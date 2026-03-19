use std::fs::{self};

use crate::commands::get_env_dir;
use crate::commands::print_ln;

pub fn list_envs() {
    let path = get_env_dir("all_envs");
    match fs::read_dir(path) {
        Ok(entries) => {
            print_ln();
            println!("Found Environments: ");
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(filename) = path.file_name() {
                        let filename = filename.to_string_lossy();
                        if filename.starts_with(".env.") {
                            println!("{}", &filename[5..]);
                        }
                    }
                }
            }
            print_ln();
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}