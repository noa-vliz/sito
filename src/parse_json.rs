use std::fs;

use serde::{Deserialize, Serialize};

use crate::paths;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Table {
    pub あ: Option<Vec<String>>,
    pub い: Option<Vec<String>>,
    pub う: Option<Vec<String>>,
    pub え: Option<Vec<String>>,
    pub お: Option<Vec<String>>,

    pub か: Option<Vec<String>>,
    pub き: Option<Vec<String>>,
    pub く: Option<Vec<String>>,
    pub け: Option<Vec<String>>,
    pub こ: Option<Vec<String>>,

    pub さ: Option<Vec<String>>,
    pub し: Option<Vec<String>>,
    pub す: Option<Vec<String>>,
    pub せ: Option<Vec<String>>,
    pub そ: Option<Vec<String>>,

    pub た: Option<Vec<String>>,
    pub ち: Option<Vec<String>>,
    pub つ: Option<Vec<String>>,
    pub て: Option<Vec<String>>,
    pub と: Option<Vec<String>>,

    pub な: Option<Vec<String>>,
    pub に: Option<Vec<String>>,
    pub ぬ: Option<Vec<String>>,
    pub ね: Option<Vec<String>>,
    pub の: Option<Vec<String>>,

    pub は: Option<Vec<String>>,
    pub ひ: Option<Vec<String>>,
    pub ふ: Option<Vec<String>>,
    pub へ: Option<Vec<String>>,
    pub ほ: Option<Vec<String>>,

    pub ま: Option<Vec<String>>,
    pub み: Option<Vec<String>>,
    pub む: Option<Vec<String>>,
    pub め: Option<Vec<String>>,
    pub も: Option<Vec<String>>,

    pub や: Option<Vec<String>>,
    pub ゆ: Option<Vec<String>>,
    pub よ: Option<Vec<String>>,

    pub ら: Option<Vec<String>>,
    pub り: Option<Vec<String>>,
    pub る: Option<Vec<String>>,
    pub れ: Option<Vec<String>>,
    pub ろ: Option<Vec<String>>,

    pub わ: Option<Vec<String>>,
}

impl Table {
    pub fn parse() -> Self {
        let config_dir = paths::config();

        let table_file = config_dir.join("table.json");

        let Ok(content) = fs::read_to_string(&table_file) else {
            eprintln!(
                "Failed to read table.json! Please create {}!",
                table_file.display()
            );
            std::process::exit(1);
        };

        let parsed: Table = serde_json::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Failed to parse file: {}!", &table_file.display());
            eprintln!("{e}");
            std::process::exit(1);
        });

        parsed
    }
}
