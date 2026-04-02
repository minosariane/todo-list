use code::{Cli, Commands, JsonStorage, Manager};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let storage = JsonStorage {
        path: "tasks.json".to_string(),
    };

    let mut manager = Manager::new(storage).unwrap_or_else(|e| {
        eprintln!("Failed to load task: {}", e);
        std::process::exit(1);
    });

    match cli.command {
        Commands::Add { title } => {
            manager.add_task(title.to_string()).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
        }
        Commands::List => {
            manager.list_task();
        }
        Commands::Done { id } => {
            manager.mark_done(id).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
        }
        Commands::Remove { id } => {
            manager.remove_task(id).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
        }
    }
}
