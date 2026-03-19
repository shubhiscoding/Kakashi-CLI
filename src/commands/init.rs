use std::fs::{self};
use std::io::Write;

use crate::commands::list_envs;
use crate::commands::print_kakashi;
pub fn init_kakashi(path: &str){
    fs::create_dir_all(".kakashi").unwrap();

    let mut config = fs::File::create(".kakashi/config").unwrap();

    writeln!(config, "env_dir={}", path).unwrap();
    print_kakashi();
    println!("Initialized kakashi with env path: {}", path);

    list_envs();
}

pub fn get_env_dir(envs: &str) -> String{
    let content = fs::read_to_string(".kakashi/config")
        .expect("Run `kakashi init <path>` first");

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