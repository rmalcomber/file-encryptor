use clap::{Parser, ValueEnum};
use path_clean::PathClean;
use std::{env, io::Write};

pub fn generate_clean_path(path: &str) -> String {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(path);

    let cleaned_path = current_dir.clean();
    match cleaned_path.to_str() {
        Some(p) => p.to_string(),
        None => panic!("Path is not a valid UTF-8 string"),
    }
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[H"); // ANSI escape code to clear screen
    std::io::stdout().flush().unwrap(); // Flush to ensure the command is sent
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Method {
    Encode,
    Decode,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_enum)]
    pub method: Method,

    /// The input file path
    pub input_path: String,

    /// The output file path
    pub output_path: String,
}
