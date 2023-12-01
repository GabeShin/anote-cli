mod config;
mod create;
mod list;
mod utils;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::parse();

    match args.command.as_str() {
        "create" => {
            create::create_note();
        }
        "configure" => {
            config::configure();
        }
        "list" => {
            list::list_notes();
        }
        "help" => {
            println!("Usage: anote <command>");
            println!("Commands:");
            println!("  create     Create a new note");
            println!("  configure  Configure anote");
            println!("  help       Display this message");
        }
        _ => println!("Usage: anote <command>"),
    }
}
