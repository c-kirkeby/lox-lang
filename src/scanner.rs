use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use anyhow::{bail, Result};

#[derive(Debug, PartialEq)]
pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
            source: source.into_bytes(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
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
            b'(' => self.add_token(TokenType::LeftParen, None),
            b')' => self.add_token(TokenType::RightParen, None),
            b'{' => self.add_token(TokenType::LeftBrace, None),
            b'}' => self.add_token(TokenType::RightBrace, None),
            b',' => self.add_token(TokenType::Comma, None),
            b'.' => self.add_token(TokenType::Dot, None),
            b'-' => self.add_token(TokenType::Minus, None),
            b'+' => self.add_token(TokenType::Plus, None),
            b';' => self.add_token(TokenType::Semicolon, None),
            b'*' => self.add_token(TokenType::Star, None),
            b'!' => {
                if self.r#match(b'=') {
                    self.add_token(TokenType::BangEqual, None)
                } else {
                    self.add_token(TokenType::Bang, None)
                }
            }
            b'=' => {
                if self.r#match(b'=') {
                    self.add_token(TokenType::Equal, None)
                } else {
                    self.add_token(TokenType::EqualEqual, None)
                }
            }
            b'<' => {
                if self.r#match(b'=') {
                    self.add_token(TokenType::Less, None)
                } else {
                    self.add_token(TokenType::LessEqual, None)
                }
            }
            b'>' => {
                if self.r#match(b'=') {
                    self.add_token(TokenType::Greater, None)
                } else {
                    self.add_token(TokenType::GreaterEqual, None)
                }
            }
            b'/' => {
                if self.peek() != b'\n' && !self.is_at_end() {
                    self.advance();
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,
            b'"' => self.string()?,
            b'0'..=b'9' => self.number()?,
            c if c.is_ascii_alphabetic() => self.identifier(),
            _ => bail!("Unexpected character on line {}", self.line),
        }
        Ok(())
    }

    fn advance(&mut self) -> u8 {
        let token = self.source[self.current];
        self.current += 1;
        token
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(
            token_type,
            String::from_utf8_lossy(text).to_string(),
            literal,
            self.line,
        ))
    }

    fn r#match(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        return self.source[self.current];
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0';
        }
        return self.source[self.current + 1];
    }

    fn string(&mut self) -> Result<()> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            bail!("Unterminated string on line {}", self.line);
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(Literal::String(value.to_vec())));
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Some(Literal::Number(
                String::from_utf8_lossy(&self.source[self.start..self.current]).parse::<f64>()?,
            )),
        );
        Ok(())
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let mut token = TokenType::Identifier;

        if let Ok(substring) = String::from_utf8(self.source[self.start..self.current].to_vec()) {
            token = match substring.as_str() {
                "and" => TokenType::And,
                "class" => TokenType::Class,
                "else" => TokenType::Else,
                "false" => TokenType::False,
                "for" => TokenType::For,
                "fun" => TokenType::Fun,
                "if" => TokenType::If,
                "nil" => TokenType::Nil,
                "or" => TokenType::Or,
                "print" => TokenType::Print,
                "return" => TokenType::Return,
                "super" => TokenType::Super,
                "this" => TokenType::This,
                "true" => TokenType::True,
                "var" => TokenType::Var,
                "while" => TokenType::While,
                _ => TokenType::Identifier,
            };
        }
        self.add_token(token, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_at_end() {
        let scanner = Scanner::new("".to_string());
        assert_eq!(scanner.is_at_end(), true);
    }

    #[test]
    fn test_scan_tokens_string() -> Result<()> {
        let mut scanner = Scanner::new("\"hello\"".to_string());
        scanner.scan_tokens()?;
        assert_eq!(
            scanner.tokens,
            vec![
                Token::new(
                    TokenType::String,
                    String::from("\"hello\""),
                    Some(Literal::String("hello".to_string().into_bytes())),
                    1
                ),
                Token::new(TokenType::EOF, String::from(""), None, 1)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_scan_tokens_number() -> Result<()> {
        let mut scanner = Scanner::new("123.45\n321".to_string());
        scanner.scan_tokens()?;
        assert_eq!(
            scanner.tokens,
            vec![
                Token::new(
                    TokenType::Number,
                    String::from("123.45"),
                    Some(Literal::Number(123.45)),
                    1
                ),
                Token::new(
                    TokenType::Number,
                    String::from("321"),
                    Some(Literal::Number(321.0)),
                    2
                ),
                Token::new(TokenType::EOF, String::from(""), None, 2)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_scan_tokens_identifier() -> Result<()> {
        let mut scanner = Scanner::new("fun hello()".to_string());
        scanner.scan_tokens()?;
        assert_eq!(
            scanner.tokens,
            vec![
                Token::new(TokenType::Fun, String::from("fun"), None, 1),
                Token::new(TokenType::Identifier, String::from("hello"), None, 1),
                Token::new(TokenType::LeftParen, String::from("("), None, 1),
                Token::new(TokenType::RightParen, String::from(")"), None, 1),
                Token::new(TokenType::EOF, String::from(""), None, 1)
            ]
        );
        Ok(())
    }
}
