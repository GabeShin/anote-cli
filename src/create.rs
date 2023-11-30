use chrono::Local;
use dialoguer::Input;
use std::{fs, path::PathBuf, process::Command};

pub fn create_note() {
    // Fetch the configuration for the save path
    let mut config_path = PathBuf::new();
    config_path = config_path
        .join(dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
        .join(".anote-config");

    let config =
        std::fs::read_to_string(config_path).unwrap_or_else(|_| "save_path=~/anote\n".to_string());

    let save_path = config
        .lines()
        .find(|line| line.starts_with("save_path="))
        .unwrap_or("save_path=~/meetingnotes")
        .replace("save_path=", "")
        .trim()
        .to_string();

    // Expand '~' to the home directory
    let expanded_path = if save_path.starts_with("~") {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(&save_path[2..])
    } else {
        PathBuf::from(&save_path)
    };

    // Create directory if it doesn't exist
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
