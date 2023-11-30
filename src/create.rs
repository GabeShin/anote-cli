use crate::utils::parse_config;
use chrono::Local;
use dialoguer::Input;
use std::{fs, path::PathBuf, process::Command};

pub fn create_note() {
    // Fetch the configuration for the save path
    let config_result = parse_config();

    // Initialize default values or use values from config
    let save_path = match config_result {
        Ok(config) => config.save_path,
        Err(e) => {
            panic!("Error reading config file: {}", e);
        }
    };

    // Create directory if it doesn't exist
    let expanded_path = PathBuf::from(&save_path);
    if !expanded_path.exists() {
        fs::create_dir_all(&expanded_path).unwrap_or_else(|e| {
            panic!("Failed to create directory: {}", e);
        });
    }

    // Get current date as default file name
    let default_file_name = Local::now().format("%Y%m%d").to_string();

    // Ask for the file name with the default
    let file_name: String = Input::new()
        .with_prompt("Enter the name of the file")
        .with_initial_text(&default_file_name)
        .interact_text()
        .unwrap();

    let full_path = expanded_path.join(file_name);

    // Check if the file already exists
    if full_path.exists() {
        println!("Error: File already exists.");
        return;
    }

    // Open the file with Vim
    match Command::new("vim")
        .arg(full_path.to_str().unwrap())
        .status()
    {
        Ok(_) => println!("Note saved."),
        Err(e) => println!("Failed to open Vim: {}", e),
    }
}
