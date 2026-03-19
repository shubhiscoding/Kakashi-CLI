use std::{fs};
use std::env;

use crate::commands::print_ln;
pub fn current_env(){
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
        if line.starts_with("current_env=") {
            print_ln();
            println!("Current environment: {}", line.replace("current_env=", ""));
            print_ln();
            return ();
        }
    }
    print_ln();
    println!("Please run kakashi switch <ENV>, atleast once to record the current env!");
    print_ln();
    panic!("Invalid config");
}