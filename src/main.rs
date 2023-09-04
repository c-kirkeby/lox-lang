use crate::scanner::*;
use std::fs;
use std::process;
use std::{io, io::prelude::*};

mod lox;
mod scanner;
mod token;
mod token_type;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        len if len > 2 => {
            println!("Usage: rs-lox [script]");
            process::exit(64)
        }
        2 => run_file(&args[0]),
        _ => run_prompt(),
    }
    Ok(())
}

fn run_file(path: &String) {
    let contents = fs::read_to_string(path).expect("Could not read file");
    run(&contents)
}

fn run(source: &String) {
    // Tokenise through scanner
    let mut scanner = Scanner::new(source.to_string());

    // let tokens = vec!["var", "language", "=", "\"lox\"", ";"];
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
}

fn run_prompt() {
    for line in io::stdin().lock().lines() {
        print!("> ");
        match line {
            Ok(line) => run(&line),
            Err(error) => {
                println!("{}", error)
            }
        }
    }
}
