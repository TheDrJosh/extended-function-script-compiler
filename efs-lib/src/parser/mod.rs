use std::{error::Error, fmt::Display};

use self::{
    lexer::Lexer,
    token::{TokenHolder, TokenType},
};

pub mod ast;
pub mod lexer;
pub mod token;
pub mod types;

pub trait ParserItem
where
    Self: Sized,
{
    fn parse(start: usize, tokens: &[TokenHolder]) -> Result<(Self, usize), ParseError>;
}

#[derive(Debug)]
pub struct ParseError {
    pub at: TokenHolder,
    pub expected: Vec<TokenType>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "parse error at {}. found: {:?}; expected: ",
            self.at.start, self.at.token
        )?;
        for i in 0..self.expected.len() {
            write!(f, "{:?}", self.expected[i])?;
            if i == self.expected.len() - 2 {
                write!(f, ", or ")?;
            } else if i != self.expected.len() - 1 {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

impl Error for ParseError {}

pub struct Parser {
    text: String,
    lexer: Lexer,
    current_token: TokenHolder,
}

impl Parser {
    pub fn new(text: String) -> anyhow::Result<Self> {
        let mut lexer = Lexer::new(text.clone());
        let current_token = lexer.next_token()?;
        Ok(Self {
            text: text,
            lexer,
            current_token,
        })
    }

    fn error(&self, expected: &[TokenType]) -> anyhow::Error {
        anyhow::anyhow!(
            "Invalid syntax, got: {:?}, expected: {:?}, at {}",
            self.current_token.token,
            expected,
            self.current_token.start
        )
    }
}
