use crate::token::Token;
use crate::token_type::TokenType;
use anyhow::{bail, Result};

#[derive(Debug, PartialEq)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
            source: source.to_string(),
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
            b'!' => {
                if self.r#match(b'=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            b'=' => {
                if self.r#match(b'=') {
                    self.add_token(TokenType::Equal)
                } else {
                    self.add_token(TokenType::EqualEqual)
                }
            }
            b'<' => {
                if self.r#match(b'=') {
                    self.add_token(TokenType::Less)
                } else {
                    self.add_token(TokenType::LessEqual)
                }
            }
            b'>' => {
                if self.r#match(b'=') {
                    self.add_token(TokenType::Greater)
                } else {
                    self.add_token(TokenType::GreaterEqual)
                }
            }
            b'/' => {
                if self.peek() != b'\n' && !self.is_at_end() {
                    self.advance();
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            _ => bail!("Unexpected character on line {}", self.line),
        }
        Ok(())
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        return self.source.as_bytes()[self.current];
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source.as_bytes()[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(
            token_type,
            String::from_utf8_lossy(text).to_string(),
            None,
            self.line,
        ))
    }

    fn r#match(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.as_bytes()[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        return self.source.as_bytes()[self.current];
    }
}
