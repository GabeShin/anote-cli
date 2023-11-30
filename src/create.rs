use std::process::Command;

pub fn open_vim_editor() {
    match Command::new("vim").status() {
        Ok(_) => println!("Note saved."),
        Err(e) => println!("Failed to open Vim: {}", e),
    }
}
