mod config;
mod create;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("create") => {
            create::create_note();
        }
        Some("configure") => {
            config::configure();
        }
        Some("help") => {
            println!("Usage: anote <command>");
            println!("Commands:");
            println!("  create     Create a new note");
            println!("  configure  Configure anote");
            println!("  help       Display this message");
        }
        _ => println!("Usage: anote <command>"),
    }
}
