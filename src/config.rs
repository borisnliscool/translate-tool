use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::string::ToString;
use std::sync::OnceLock;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub post_write_commands: Vec<String>,
    pub default_locale: String,
    pub translations_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_locale: "en".to_string(),
            post_write_commands: Vec::new(),
            translations_directory: "translations".to_string(),
        }
    }
}

fn load_config(config_path: Option<String>) -> Result<Config, String> {
    let config_path = config_path.unwrap_or_else(|| "tt.config.json".to_string());
    let config_path = Path::new(&config_path);

    if !config_path.exists() {
        return Err("Config not found".to_string());
    }

    let file = File::open(config_path.to_str().unwrap());
    if file.is_err() {
        return Err(format!(
            "Could not open file {}",
            config_path.to_str().unwrap()
        ));
    }

    let mut file = file.unwrap();
    let mut contents = String::new();

    if let Err(_) = file.read_to_string(&mut contents) {
        return Err(format!(
            "Could not read file {}",
            config_path.to_str().unwrap()
        ));
    }

    let deserialized = serde_json::from_str::<Config>(&contents);
    if deserialized.is_err() {
        return Err(format!(
            "Could not deserialize file {}: {}",
            config_path.to_str().unwrap(),
            deserialized.unwrap_err()
        ));
    }

    Ok(deserialized.unwrap())
}

pub fn get_config(config_path: Option<String>) -> Config {
    CONFIG
        .get_or_init(|| load_config(config_path).unwrap_or_default())
        .to_owned()
}

pub fn run_after_write_commands() {
    let config = CONFIG.get();

    if config.is_none() {
        // todo: decide if we want to do something here
        return;
    }

    let config = config.unwrap();
    let cwd = current_dir().unwrap();

    for command in config.post_write_commands.to_owned() {
        println!("Executing: \"{}\"\n", command);

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command.as_str()])
                .current_dir(cwd.as_path())
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(cwd.as_path())
                .output()
                .expect("failed to execute process")
        };

        if output.stderr.len() > 0 {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        } else {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
