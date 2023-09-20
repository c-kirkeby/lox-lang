use crate::scanner::*;
use anyhow::Result;
use std::fs;
use std::process;

use std::io::{self, Write};

#[derive(Debug, PartialEq)]
pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &String) -> Result<()> {
        let contents = fs::read_to_string(path).expect("Could not read file");

        match Self::run(contents) {
            Err(_) => {
                self.had_error = true;
            }
            Ok(_) => (),
        };

        if self.had_error {
            process::exit(65);
        };
        Ok(())
    }

    pub fn run(source: String) -> Result<()> {
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens()?;
        for token in tokens {
            println!("{}", token);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Could not read line");
            if line.is_empty() {
                break;
            }

            Self::run(line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_lox() {
        let a = Lox::new();
        let b = Lox { had_error: false };

        assert_eq!(a, b);
    }

    #[test]
    fn test_run() {
        let a = "*\n".to_string();

        let _ = Lox::run(a);
    }
}
