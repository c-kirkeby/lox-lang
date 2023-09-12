use crate::token::Token;
use crate::token_type::TokenType;
use anyhow::{bail, Result};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u8,
    current: u8,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
            source,
        }
    }

    fn is_at_end(&self) -> bool {
        usize::from(self.current) >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Result<&[Token]> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            None,
            self.line,
        ));

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<()> {
        let token = self.advance();

        match token {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),
            _ => bail!("Unexpected character on line {}", self.line),
        }
        Ok(())
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.current
    }

    #[allow(unused)]
    fn add_token(&self, token_type: TokenType) {}
}
