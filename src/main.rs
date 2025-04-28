use parse_json::Table;
use rustyline::error::ReadlineError;

mod init;
mod parse_json;
mod paths;
mod sito;

fn print_welcome_message() {
    println!("=== Welcome to the Shiritori Game! ===");
    println!("Rules:");
    println!("- Input a word that starts with the last character of the previous word");
    println!("- If you use a word ending with 'ん', you lose");
    println!("- Type 'exit' to end the game");
    println!("- The first word can be anything you like!");
    println!("Let's begin!\n");
}

fn is_valid_japanese(input: &str) -> bool {
    input.chars().all(|c| match c {
        'あ'..='ん' => true,
        _ => false,
    })
}

fn main() {
    init::init();

    let mut rl = rustyline::DefaultEditor::new().unwrap_or_else(|_e| {
        eprintln!("Could not initialize the input handler");
        std::process::exit(1);
    });
    print_welcome_message();

    let table = Table::parse();

    let mut count = 0u64;

    let mut previous_word_ending: char = ' ';
    let mut is_invalid_word = false;

    loop {
        count += 1;
        let input = match rl.readline("You> ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => {
                println!("\nThank you for playing the game!");
                eprintln!("You completed {} turns in total!", count - 1);
                return;
            }
            Err(ReadlineError::Eof) => {
                println!("\nThank you for playing the game!");
                eprintln!("You completed {} turns in total!", count - 1);
                return;
            }
            Err(err) => {
                eprintln!("An error occurred: {}", err);
                continue;
            }
        };

        let input_word = input.trim();

        let _ = rl.add_history_entry(input_word);

        if input_word == "exit" {
            eprintln!("You completed {} turns in total!", count);
            return;
        }

        if !is_valid_japanese(input_word) {
            println!("Please input using hiragana characters only!");
            is_invalid_word = true;
            continue;
        }

        if input_word.len() < 2 {
            println!("Please enter a word with at least 2 characters!");
            is_invalid_word = true;
            continue;
        }

        let characters = input_word.chars().collect::<Vec<char>>();

        if characters.last().unwrap() == &'ん' {
            eprintln!("Game Over!");
            eprintln!(
                "You used a word ending with 'ん'. You played with {} words in total!",
                count
            );
            return;
        }

        if count > 1 && !is_invalid_word {
            if previous_word_ending != characters.first().unwrap().clone() {
                eprintln!("Game Over!");
                eprintln!(
                    "Your word doesn't start with the last character of the previous word. You played with {} words in total!",
                    count
                );
                return;
            }
        }

        let response = sito::sito(input_word, table.clone());

        if let Some(res) = response {
            let response_chars = res.chars().collect::<Vec<char>>();
            previous_word_ending = response_chars.last().unwrap().clone();
            println!("Computer> {}", res);

            if previous_word_ending == 'ん' {
                eprintln!("You Win!");
                eprintln!(
                    "The computer used a word ending with 'ん'. You played with {} words in total!",
                    count
                );
                return;
            }

            is_invalid_word = false;
        } else {
            is_invalid_word = true;
        }
    }
}
