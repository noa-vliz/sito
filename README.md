# Sito - しりとり (Shiritori) Game

A command-line Japanese word chain game (しりとり/shiritori) implemented in Rust.

## What is Shiritori?

Shiritori is a traditional Japanese word game where players take turns saying words that begin with the last character of the previous word. The name "shiritori" (しりとり) literally means "taking the rear" (しり means "buttocks" or "rear" and とる means "to take").

## Features

- Play shiritori against the computer
- Japanese hiragana character support
- Word database expandable via JSON
- Command-line interface with history support
- Automatic game initialization with sample words
- Game tracks number of turns played

## Requirements

- Rust (2024 edition or newer)
- Cargo (Rust's package manager)

## Installation

Clone this repository:

```bash
git clone https://github.com/noa-vliz/sito.git
cd sito
```

Build the project:

```bash
cargo build --release
```

Run the application:

```bash
cargo run --release
```

## How to Play

1. Launch the application
2. Type a Japanese word in hiragana when prompted with `>`
3. The computer will respond with a word starting with the last character of your word
4. Continue the chain by entering a word that starts with the last character of the computer's word
5. Type `exit` at any time to quit the game

## Game Rules

1. Words must be in hiragana (Japanese syllabary)
2. Each new word must begin with the last character of the previous word
3. The game ends in the following cases:
   - When a player uses a word ending with 'ん' (since no Japanese word begins with 'ん')
   - When a player fails to continue the chain with a valid word
   - When a player types `exit`

## Word Database

The game uses a JSON file to store its word database. On first run, a sample database is created at:

- Linux/macOS: `~/.config/sito/table.json`
- Windows: `%APPDATA%\sito\table.json`

You can edit this file to add more words. The database is organized by the first character of each word.

## Dependencies

- dirs-next: For finding configuration directories
- rand: For random word selection
- rustyline: For command-line input with history
- serde/serde_json: For JSON processing

## License

This software is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

