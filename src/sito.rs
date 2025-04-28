use rand::seq::IndexedRandom;

use crate::parse_json::Table;

pub fn sito<T: AsRef<str>>(word: T, mut table: Table) -> Option<String> {
    let word = word.as_ref();

    let chars = word.trim().chars().collect::<Vec<char>>();

    if chars.is_empty() {
        return None;
    }

    let last_word = chars.last().unwrap();

    let mut word_table = None;

    match last_word {
        'あ' => word_table = table.あ,
        'い' => word_table = table.い,
        'う' => word_table = table.う,
        'え' => word_table = table.え,
        'お' => word_table = table.お,

        'か' => word_table = table.か,
        'き' => word_table = table.き,
        'く' => word_table = table.く,
        'け' => word_table = table.け,
        'こ' => word_table = table.こ,

        'さ' => word_table = table.さ,
        'し' => word_table = table.し,
        'す' => word_table = table.す,
        'せ' => word_table = table.せ,
        'そ' => word_table = table.そ,

        'た' => word_table = table.た,
        'ち' => word_table = table.ち,
        'つ' => word_table = table.つ,
        'て' => word_table = table.て,
        'と' => word_table = table.と,

        'な' => word_table = table.な,
        'に' => word_table = table.に,
        'ぬ' => word_table = table.ぬ,
        'ね' => word_table = table.ね,
        'の' => word_table = table.の,

        'は' => word_table = table.は,
        'ひ' => word_table = table.ひ,
        'ふ' => word_table = table.ふ,
        'へ' => word_table = table.へ,
        'ほ' => word_table = table.ほ,

        'ま' => word_table = table.ま,
        'み' => word_table = table.み,
        'む' => word_table = table.む,
        'め' => word_table = table.め,
        'も' => word_table = table.も,

        'や' => word_table = table.や,
        'ゆ' => word_table = table.ゆ,
        'よ' => word_table = table.よ,

        'ら' => word_table = table.ら,
        'り' => word_table = table.り,
        'る' => word_table = table.る,
        'れ' => word_table = table.れ,
        'ろ' => word_table = table.ろ,

        'わ' => word_table = table.わ,

        _ => {
            eprintln!("Unknown char: {}", &last_word);
            return None;
        }
    }

    if word_table.is_none() {
        return None;
    }

    if let Some(word_table) = word_table {
        let p = word_table.choose(&mut rand::rng());

        if p.is_some() {
            return Some(p.unwrap().clone());
        }
    }

    return None;
}
