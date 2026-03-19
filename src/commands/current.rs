use std::{fs};
pub fn current_env(){
    let content = fs::read_to_string(".kakashi/config")
        .expect("Run `kakashi init <path>` first");
    
    for line in content.lines() {
        if line.starts_with("current_env=") {
            println!("Current environment: {}", line.replace("current_env=", ""));
            return ();
        }
    }

    panic!("Invalid config");
}