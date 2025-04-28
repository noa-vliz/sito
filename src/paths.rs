use std::path::PathBuf;

pub fn config() -> PathBuf {
    if let Some(config_dir) = dirs_next::config_dir() {
        config_dir.join("sito")
    } else {
        eprintln!("!!!! Failed to get config directory !!!!");
        std::process::exit(1);
    }
}
