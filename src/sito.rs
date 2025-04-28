use rand::seq::IndexedRandom;

use crate::parse_json::Table;

pub fn sito<T: AsRef<str>>(word: T, table: Table) -> Option<String> {
    let word = word.as_ref();

    let chars = word.trim().chars().collect::<Vec<char>>();

    if chars.is_empty() {
        return None;
    }

    let last_word = chars.last().unwrap();

    let word_table = match last_word {
        'あ' => table.あ,
        'い' => table.い,
        'う' => table.う,
        'え' => table.え,
        'お' => table.お,

        'か' => table.か,
        'き' => table.き,
        'く' => table.く,
        'け' => table.け,
        'こ' => table.こ,

        'さ' => table.さ,
        'し' => table.し,
        'す' => table.す,
        'せ' => table.せ,
        'そ' => table.そ,

        'た' => table.た,
        'ち' => table.ち,
        'つ' => table.つ,
        'て' => table.て,
        'と' => table.と,

        'な' => table.な,
        'に' => table.に,
        'ぬ' => table.ぬ,
        'ね' => table.ね,
        'の' => table.の,

        'は' => table.は,
        'ひ' => table.ひ,
        'ふ' => table.ふ,
        'へ' => table.へ,
        'ほ' => table.ほ,

        'ま' => table.ま,
        'み' => table.み,
        'む' => table.む,
        'め' => table.め,
        'も' => table.も,

        'や' => table.や,
        'ゆ' => table.ゆ,
        'よ' => table.よ,

        'ら' => table.ら,
        'り' => table.り,
        'る' => table.る,
        'れ' => table.れ,
        'ろ' => table.ろ,

        'わ' => table.わ,

        _ => {
            eprintln!("Sorry, I don't know any words starting with '{}'...", &last_word);
            return None;
        }
    };

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
