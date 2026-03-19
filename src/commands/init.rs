use std::fs::{self};
use std::io::Write;
use std::env;
use std::path::Path;
use crate::commands::{print_ln};
use crate::commands::print_kakashi;
pub fn init_kakashi(path: &str){
    let path = path.trim_matches(|c| c == '/' || c == '\\');
    let curr_path = env::current_dir().unwrap();
    let project_path = curr_path.join(Path::new(path));
    let kakashi_dir = format!("{}/.kakashi", path);
    fs::create_dir_all(&kakashi_dir).unwrap();

    let mut config = fs::File::create(format!("{}/config", kakashi_dir)).unwrap();

    writeln!(config, "env_dir={}", ".").unwrap();
    writeln!(config, "project_path={}", project_path.display()).unwrap();

    print_kakashi();
    print_ln();
    if path != "."{
        println!("Initialized kakashi with env path: {}", project_path.display());
        println!("Run cd {}", path);
    }else {
        println!("Initialized kakashi with env path: {}", curr_path.display());
    }
    print_ln();
}

pub fn get_env_dir(envs: &str) -> String{
    let content = fs::read_to_string(".kakashi/config")
        .expect("Run `kakashi init <path>` first, or move to the correct directory");

    let curr_path = env::current_dir().unwrap().display().to_string();
    
    if content.is_empty() {
        println!("Please Run `kakashi init <path>` or move to the correct path");
    }
    
    for line in content.lines() {
        if line.starts_with("project_path=") {
            let saved_path = line.replace("project_path=", "");
            if saved_path != curr_path {
                panic!("Please run init or move to the correct path");
            }
        }
    }

    for line in content.lines() {
        if line.starts_with("env_dir=") {
            match envs {
                "all_envs" => {return line.replace("env_dir=", "");}
                "" => {return format!("{}/.env", line.replace("env_dir=", ""));}
                _ => {return format!("{}/.env.{}", line.replace("env_dir=", ""), envs);}
            }
        }
    }

    panic!("Invalid config");
}