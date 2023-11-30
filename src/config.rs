use crate::utils::parse_config;
use dialoguer::{Confirm, Input};
use std::fs;

pub fn configure() {
    // Fetch the configuration for the save path
    let config_result = parse_config();

    // Initialize default values or use values from config
    let (save_path, openai_key, config_path) = match config_result {
        Ok(config) => (config.save_path, config.openai_key, config.config_path),
        Err(e) => {
            panic!("Error reading config file: {}", e);
        }
    };

    // Prompt for save path
    let save_path: String = Input::new()
        .with_prompt("Enter the save path for notes")
        .default(save_path.clone())
        .interact_text()
        .unwrap();

    // Prompt for OpenAI key
    let openai_key: String = Input::new()
        .with_prompt("Enter OpenAI Key")
        .default(openai_key.clone())
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
