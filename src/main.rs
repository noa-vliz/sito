use parse_json::Table;

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

    loop {
        count += 1;
        let input = rl.readline("> ").unwrap();

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

        if count > 1 {
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
        }
    }
}
