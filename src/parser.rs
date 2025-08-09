use serde_json::{Map, Value};
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ParserError {
    FileDoesNotExist(PathBuf),
    CouldNotOpenFile(PathBuf),
    CouldNotParseFile(PathBuf),
    InvalidValueType { key: String, value_type: String },
    Generic(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = match self {
            ParserError::FileDoesNotExist(path) => {
                format!("File does not exist: {}", path.display())
            }
            ParserError::CouldNotOpenFile(path) => {
                format!("Could not open file: {}", path.display())
            }
            ParserError::CouldNotParseFile(path) => {
                format!("Could not parse file: {}", path.display())
            }
            ParserError::InvalidValueType { key, value_type } => {
                format!(
                    "Invalid value '{}' for key: '{}', it should be either a string or map with strings.",
                    value_type, key
                )
            }
            ParserError::Generic(error) => error.clone(),
        };
        write!(f, "{}", str)
    }
}

pub fn get_parsed_translation_file(path: PathBuf) -> Result<Value, ParserError> {
    if !path.exists() {
        return Err(ParserError::FileDoesNotExist(path));
    }

    let file = File::open(path.to_str().unwrap());
    if file.is_err() {
        return Err(ParserError::CouldNotOpenFile(path));
    }

    let mut file = file.unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let parsed = serde_json::from_str(&contents);
    if parsed.is_err() {
        return Err(ParserError::CouldNotParseFile(path));
    }

    Ok(parsed.unwrap())
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectKeyOption {
    ExcludeObjectKeys,
    OnlyObjectKeys,
}

pub fn get_translation_keys(
    value: Value,
    base: String,
    object_key_option: ObjectKeyOption,
) -> Result<Vec<String>, ParserError> {
    match value {
        Value::String(_) => Ok(vec![]),
        Value::Object(map) => {
            let mut keys = Vec::new();

            for (key, sub_value) in map {
                if object_key_option == ObjectKeyOption::OnlyObjectKeys {
                    match sub_value {
                        Value::Object(_) => {}
                        _ => continue,
                    }
                }

                let sub_base = if base.is_empty() {
                    key
                } else {
                    format!("{}.{}", base, key)
                };

                let sub_keys =
                    get_translation_keys(sub_value, sub_base.clone(), object_key_option.clone())?;

                if sub_keys.is_empty() {
                    keys.push(sub_base);
                }

                keys.extend(sub_keys);
            }

            Ok(keys)
        }
        _ => Err(ParserError::InvalidValueType {
            key: base,
            value_type: format!("{:?}", value),
        }),
    }
}

pub fn update_translation_key(
    mut value: Value,
    key: String,
    updated_value: String,
) -> Result<Value, ParserError> {
    let split = key.split('.').collect::<Vec<&str>>();

    if split.len() < 1 {
        return Err(ParserError::Generic("Invalid key".to_string()));
    }

    let Some(mut current) = value.as_object_mut() else {
        return Err(ParserError::Generic(
            "value should be a JSON Object".to_string(),
        ));
    };

    for part in &split[..split.len() - 1] {
        if !current.contains_key(*part) {
            current.insert(part.to_string(), Value::Object(Map::new()));
        }

        let Some(curr) = current.get_mut(*part) else {
            return Err(ParserError::Generic(
                "should have a mutable reference to the next level".to_string(),
            ));
        };

        let Some(curr) = curr.as_object_mut() else {
            return Err(ParserError::Generic(
                "next level should be a JSON Object".to_string(),
            ));
        };

        current = curr;
    }

    current.insert(
        split.last().unwrap().to_string(),
        Value::String(updated_value),
    );

    Ok(value)
}

pub fn get_translation_value(value: Value, key: String) -> Result<String, ParserError> {
    let split = key.split('.').collect::<Vec<&str>>();

    if split.len() < 1 {
        return Err(ParserError::Generic("Invalid key".to_string()));
    }

    let mut current = &value;

    for part in &split[..split.len() - 1] {
        if let Some(next) = current.get(part) {
            current = next;
        } else {
            return Err(ParserError::Generic(format!("Key '{}' not found", part)));
        }
    }

    let last_part = split.last();
    if last_part.is_none() {
        return Err(ParserError::Generic(format!("Key '{}' not found", key)));
    }

    let last_part = last_part.unwrap();

    if let Some(last_value) = current.get(last_part) {
        if let Some(string_value) = last_value.as_str() {
            Ok(string_value.to_string())
        } else {
            Err(ParserError::Generic(format!(
                "Value for key '{}' is not a string",
                last_part
            )))
        }
    } else {
        Err(ParserError::Generic(format!(
            "Key '{}' not found",
            last_part
        )))
    }
}
