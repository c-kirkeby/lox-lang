use crate::token_type;
use std::fmt;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Literal {
    String(Vec<u8>),
    Number(f64),
    None,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: token_type::TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: token_type::TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
