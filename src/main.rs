use crate::lox::Lox;
use anyhow::Result;
use std::process;

mod lox;
mod scanner;
mod token;
mod token_type;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut interpreter = Lox::new();

    match args.len() {
        1 => interpreter.run_prompt()?,
        2 => interpreter.run_file(&args[1])?,
        _ => {
            println!("Usage: rs-lox [script]");
            process::exit(64)
        }
    }
    Ok(())
}
