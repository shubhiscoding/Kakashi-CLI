use std::{fs::{self, File, OpenOptions}, io::Write};

use crate::commands::get_env_dir;

pub fn switch_to_env(switch_to: String) {
    let path = get_env_dir(&switch_to);
    let env_path = get_env_dir("");

    let content = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("{} environment not found", switch_to));

    let mut env_file = File::create(&env_path)
        .expect("Failed to open .env file for writing");

    env_file
        .write_all(content.as_bytes())
        .expect("Failed to write to .env file");

    let mut config = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".kakashi/config")
        .unwrap();

    writeln!(config, "current_env={}", switch_to).unwrap();
}
