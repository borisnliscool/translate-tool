use serde_json::Value;
use crate::cli::CommandArgs;
use crate::commands::CommandError;
use crate::parser::ObjectKeyOption;
use crate::{files, parser};

pub fn validate_command(args: CommandArgs, fail_on_empty: bool) -> Result<(), CommandError> {
    let (default_locale_path, translation_files) =
        files::get_translation_files(args.translations_directory.clone())
            .map_err(|e| CommandError::Generic(e.to_string()))?;

    let default_translation_value = parser::get_parsed_translation_file(default_locale_path.into())
        .map_err(|e| CommandError::Generic(e.to_string()))?;

    let default_translation_keys = parser::get_translation_keys(
        default_translation_value.clone(),
        "".to_string(),
        ObjectKeyOption::ExcludeObjectKeys,
    )
    .map_err(|e| CommandError::Generic(e.to_string()))?;

    let mut issues: Vec<String> = Vec::new();

    for file_path in translation_files {
        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        let translation_value = parser::get_parsed_translation_file(file_path.into())
            .map_err(|e| CommandError::Generic(e.to_string()))?;

        let translation_keys = parser::get_translation_keys(
            translation_value.clone(),
            "".to_string(),
            ObjectKeyOption::ExcludeObjectKeys,
        )
        .map_err(|e| CommandError::Generic(e.to_string()))?;

        for key in default_translation_keys.clone() {
            if !translation_keys.contains(&key) {
                issues.push(format!("{} is missing key '{}'", file_name, key));
            }
        }

        if fail_on_empty {
            let empty_values = get_empty_values(file_name, &translation_value, "".to_string());
            issues.extend(empty_values.clone());
        }
    }
    
    let issues = issues.iter();
    let issue_count = issues.len();
    
    if issue_count > 0 {
        for issue in issues {
            eprintln!("{}", issue);
        }
        
        return Err(CommandError::Generic(format!("{} issues found.", issue_count)));
    }
    
    println!("All keys present!");

    Ok(())
}

fn get_empty_values(file_name: String, value: &Value, key: String) -> Vec<String> {
    let mut issues: Vec<String> = Vec::new();

    match value {
        Value::String(val) => {
            if val.is_empty() {
                issues.push(format!("{} has an empty value '{}'", file_name, key));
            }
        },
        Value::Object(val) => {
            for (key, value) in val {
                issues.extend(
                    get_empty_values(file_name.clone(), value, key.to_owned())
                );
            }
        },
        _ => ()
    }

    issues
}