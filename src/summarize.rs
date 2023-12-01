use crate::utils::{parse_config, run_command};
use std::{fs, path::PathBuf, process::Command};

pub fn summarize(full_path: PathBuf) {
    let config_result = parse_config();
    let save_path = match config_result {
        Ok(config) => config.save_path,
        Err(e) => {
            panic!("Error reading config file: {}", e);
        }
    };

    // Read the content of the note
    let note_content = fs::read_to_string(&full_path).expect("Failed to read the file");

    // Run the 'ollama' command with the note content
    // TODO: model name can be customized
    let ollama_command = format!(
        "ollama run mistral \"Format the note to markdown file: {}\"",
        note_content
    );

    let summary = match run_command(&ollama_command) {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => {
            println!("Failed to execute ollama command: {}", e);
            return;
        }
    };

    // Create the summary file path
    let file_stem = full_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("default");
    let summary_file_name = format!("{}-summary.md", file_stem);
    let summary_file_path = PathBuf::from(&save_path).join(summary_file_name);

    // Save the summary to the file
    fs::write(&summary_file_path, &summary).expect("Failed to write the summary file");

    // Open the Markdown file with a default application or editor
    if let Err(e) = Command::new("vim")
        .arg(summary_file_path.to_str().unwrap())
        .status()
    {
        println!("Failed to open the Markdown file: {}", e);
    }
}
