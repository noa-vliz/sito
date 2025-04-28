use std::path::PathBuf;

pub fn config() -> PathBuf {
    if let Some(config_dir) = dirs_next::config_dir() {
        config_dir.join("sito")
    } else {
        eprintln!("Error: Unable to find the configuration directory. Please ensure you have appropriate permissions.");
        std::process::exit(1);
    }
}
