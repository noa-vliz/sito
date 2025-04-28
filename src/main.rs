use parse_json::Table;
use rustyline::error::ReadlineError;

mod init;
mod parse_json;
mod paths;
mod sito;

fn main() {
    init::init();

    let mut rl = rustyline::DefaultEditor::new().unwrap_or_else(|_e| {
        eprintln!("Failed to initialize read line!");
        std::process::exit(1);
    });

    let table = Table::parse();

    let mut count = 0u64;

    let mut last_response_char: char = ' ';
    let mut unknown = false;

    loop {
        count += 1;
        let input = match rl.readline("> ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => {
                println!("\nGame interrupted. Goodbye!");
                eprintln!("A total of {} times completed!", count - 1);
                return;
            },
            Err(ReadlineError::Eof) => {
                println!("\nEnd of input. Goodbye!");
                eprintln!("A total of {} times completed!", count - 1);
                return;
            },
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };

        let trm = input.trim();

        if trm == "exit" {
            eprintln!("A total of {} times completed!", count);
            return;
        }

        let ch = trm.chars().collect::<Vec<char>>();

        if ch.last().unwrap() == &'ん' {
            eprintln!("Finish!");
            eprintln!("The shiritori game ended on the {}th time!", count);
            return;
        }

        if count > 1 && !unknown {
            if last_response_char != ch.first().unwrap().clone() {
                eprintln!("Finish!");
                eprintln!("The shiritori game ended on the {}th time!", count);
                return;
            }
        }

        let response = sito::sito(trm, table.clone());

        if let Some(res) = response {
            let bind = res.chars().collect::<Vec<char>>();
            last_response_char = bind.last().unwrap().clone();
            println!("{}", res);

            if last_response_char == 'ん' {
                eprintln!("Finish!");
                eprintln!("The shiritori game ended on the {}th time!", count);
                return;
            }

            unknown = false;
        } else {
            unknown = true;
        }
    }
}
