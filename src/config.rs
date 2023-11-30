use std::{fs, io::ErrorKind, path::PathBuf};

use dialoguer::{Confirm, Input};

pub fn configure() {
    // Path to the config file
    let mut config_path = PathBuf::new();
    config_path = config_path
        .join(dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
        .join(".anote-config");

    // Initialize default values
    let mut default_save_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("anote")
        .to_str()
        .unwrap()
        .to_string();
    let mut default_openai_key = "User".to_string();

    // Check if config file exists and read values
    match fs::read_to_string(&config_path) {
        Ok(contents) => {
            for line in contents.lines() {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "save_path" => default_save_path = parts[1].to_string(),
                        "openai_key" => default_openai_key = parts[1].to_string(),
                        _ => {}
                    }
                }
            }
        }
        Err(e) if e.kind() != ErrorKind::NotFound => {
            eprintln!("Error reading config file: {}", e);
            return;
        }
        _ => {}
    }

    // Prompt for save path
    let save_path: String = Input::new()
        .with_prompt("Enter the save path for notes")
        .default(default_save_path.clone())
        .interact_text()
        .unwrap();

    // Prompt for OpenAI key
    let openai_key: String = Input::new()
        .with_prompt("Enter OpenAI Key")
        .default(default_openai_key.clone())
        .interact_text()
        .unwrap();

    // Confirm the inputs
    if Confirm::new()
        .with_prompt("Are these details correct?")
        .interact()
        .unwrap()
    {
        // Save the configuration
        let config_content = format!("save_path={}\nopenai_key={}", save_path, openai_key);
        fs::write(config_path, config_content).expect("Unable to write config file");

        println!("Configuration saved successfully.");
    } else {
        println!("Configuration not saved. Please run 'anote configure' again to set up.");
    }
}
