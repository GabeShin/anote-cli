use std::fs;
use std::io::{ErrorKind, Result};
use std::path::PathBuf;

pub fn parse_config() -> Result<Config> {
    let config_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".anote-config");

    let default_save_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("anote")
        .to_str()
        .unwrap()
        .to_string();

    let default_openai_key = "openai-key".to_string();

    let mut config = Config {
        save_path: default_save_path,
        openai_key: default_openai_key,
        config_path: config_path.to_str().unwrap().to_string(),
    };

    match fs::read_to_string(&config_path) {
        Ok(contents) => {
            for line in contents.lines() {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "save_path" => config.save_path = parts[1].to_string(),
                        "openai_key" => config.openai_key = parts[1].to_string(),
                        _ => {}
                    }
                }
            }
        }
        Err(e) if e.kind() != ErrorKind::NotFound => {
            eprintln!("Error reading config file: {}", e);
            return Err(e);
        }
        _ => {}
    }

    Ok(config)
}

pub struct Config {
    pub save_path: String,
    pub openai_key: String,
    pub config_path: String,
}

