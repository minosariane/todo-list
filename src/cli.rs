use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "over-engineerd cli todo-list")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { title: String },
    List,
    Done { id: usize },
    Remove { id: usize },
}
