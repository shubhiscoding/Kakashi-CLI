use std::{fs};

use crate::commands::print_ln;
pub fn current_env(){
    let content = fs::read_to_string(".kakashi/config")
        .expect("Run `kakashi init <path>` first");
    
    for line in content.lines() {
        if line.starts_with("current_env=") {
            print_ln();
            println!("Current environment: {}", line.replace("current_env=", ""));
            print_ln();
            return ();
        }
    }

    panic!("Invalid config");
}