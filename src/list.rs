use crate::utils::parse_config;
use std::{fs, path::Path};

pub fn list_notes() {
    // Fetch the configuration for the save path
    let config_result = parse_config();

    // Initialize default values or use values from config
    let save_path = match config_result {
        Ok(config) => config.save_path,
        Err(e) => {
            panic!("Error reading config file: {}", e);
        }
    };

    // Ensure the path exists and is a directory
    if !Path::new(&save_path).is_dir() {
        panic!("Save path is not a directory: {}", save_path);
    }

    // Read the directory and list the files
    match fs::read_dir(&save_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(file) => {
                        let path = file.path();
                        if path.is_file() {
                            // You might want to handle file names that are not valid UTF-8
                            println!("{}", path.file_name().unwrap().to_string_lossy());
                        }
                    }
                    Err(e) => println!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => panic!("Error reading directory: {}", e),
    }
}
