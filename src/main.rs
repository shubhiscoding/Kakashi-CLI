use clap::{Parser, Subcommand};

mod commands;
use commands::init_kakashi;
use commands::list_envs;
use commands::switch_to_env;

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
            init_kakashi(&path);
        }
        Commands::List => {
            list_envs();
        }
        Commands::Switch { env } => {
            switch_to_env(env);
        }
        Commands::Current => {
            println!("Current env...");
        }
    }
}

