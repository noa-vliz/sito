use std::{fs, io::Write};

use crate::paths;

const SAMPLE_TABLE: &[u8] = include_bytes!("../table_sample.json");

pub fn init() {
    let config_dir = paths::config();

    if !config_dir.exists() {
        if fs::create_dir_all(&config_dir).is_ok() {
            eprintln!("\x1b[33;1mCreated\x1b[0;1m {}", &config_dir.display());
        }
    }

    let table_path = config_dir.join("table.json");

    if !table_path.exists() {
        if let Ok(mut file) = fs::File::create(&table_path) {
            if file.write_all(SAMPLE_TABLE).is_ok() {
                eprintln!("\x1b[33;1mCreated\x1b[0;1m {}", &table_path.display());
            } else {
                eprintln!(
                    "\x1b[31;1m\x1b[0;1m Could not create configuration file: {}",
                    &table_path.display()
                );
            }
        }
    }
}
