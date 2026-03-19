use clap::{Parser, Subcommand};

mod commands;
use commands::init;

#[derive(Parser)]
#[command(name = "kakashi")]
#[command(about = "Smart env manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { path: String },
    List,
    Switch { env: String },
    Current,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            init(&path);
        }
        Commands::List => {
            println!("Listing envs...");
        }
        Commands::Switch { env } => {
            println!("Switching to {}", env);
        }
        Commands::Current => {
            println!("Current env...");
        }
    }
}

