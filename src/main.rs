use std::{
    fs::{self, File},
    io::Write,
};

use clap::Parser;
use lib::{decode_bytes, encode_bytes};
use rpassword::read_password;

mod helpers;

fn main() {
    let args = helpers::Args::parse();

    let mut password = String::new();
    println!("Enter a password:");
    match read_password() {
        Ok(p) => password = p,
        Err(_) => println!("Error reading password"),
    }

    let input_path = helpers::generate_clean_path(&args.input_path);
    let output_path = helpers::generate_clean_path(&args.output_path);

    helpers::clear_terminal();
    match args.method {
        helpers::Method::Encode => encode(input_path, output_path, password),
        helpers::Method::Decode => decode(input_path, output_path, password),
    }
}

fn get_file(path: String) -> Vec<u8> {
    match fs::read(path) {
        Ok(bytes) => bytes,
        Err(_) => panic!("Failed to read file"),
    }
}

fn encode(input_path: String, output_path: String, password: String) {
    println!("Encoding {} -> {}", input_path, output_path);

    let bytes = get_file(input_path);

    let encoded = encode_bytes(bytes, password);

    let mut file = File::create(output_path).unwrap();

    match file.write_all(&encoded) {
        Ok(_) => println!("File written successfully"),
        Err(_) => panic!("Failed to write file"),
    }
}

fn decode(input_path: String, output_path: String, password: String) {
    println!("Decoding {} -> {}", input_path, output_path);

    let bytes = get_file(input_path);

    let decoded = decode_bytes(bytes, password);

    let mut file = File::create(output_path).unwrap();

    match file.write_all(&decoded) {
        Ok(_) => println!("File written successfully"),
        Err(_) => panic!("Failed to write file"),
    }
}
