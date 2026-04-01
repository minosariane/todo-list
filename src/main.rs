mod modules;

use std::env;

use crate::modules::{manager::Manager, json_storage::JsonStorage};

fn main() {
    let storage = JsonStorage {
        path: "tasks.json".to_string(),
    };

    let mut manager = Manager::new(storage)
        .unwrap_or_else(|e| {
            eprintln!("Failed to load task: {}", e);
            std::process::exit(1);
        });

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            if let Some(title) = args.get(2) {
                manager.add_task(title.to_string()).unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                });
            }
        }
        Some("list") => manager.list_task(),
        Some("done") => {
            if let Some(id) = args.get(2) {
                let id = id.parse().unwrap();
                manager.mark_done(id).unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                });
            }
        }
        Some("remove") => {
            if let Some(id) = args.get(2) {
                let id = id.parse().unwrap();
                manager.remove_task(id).unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                });
            }
        }
        _ => println!("Unkown command"),
    }
}
