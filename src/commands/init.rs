use std::fs::{self, DirEntry};
use std::io::Write;

fn read_envs(path: &str) {
    match fs::read_dir(path) {
        Ok(entries) => {
        println!("Found: ");
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(filename) = path.file_name() {
                        let filename = filename.to_string_lossy();
                        if filename.starts_with(".env.") {
                            println!("{}", filename);
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}

pub fn init(path: &str){
    fs::create_dir_all(".kakashi").unwrap();

    let mut config = fs::File::create(".kakashi/config").unwrap();

    writeln!(config, "env_dir={}", path).unwrap();
    println!("Initialized kakashi with env path: {}", path);

    read_envs(path);
}