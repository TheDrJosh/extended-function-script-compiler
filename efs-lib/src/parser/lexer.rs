use std::{error::Error, fmt::Display};

use crate::parser::token::Token;

use super::token::TokenHolder;

pub struct Lexer {
    text: Vec<char>,
    pos: usize,
    history: Vec<TokenHolder>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text: text.chars().collect(),
            pos: 0,
            history: Vec::new() ,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.text.get(self.pos).map_or(false, |c| c.is_whitespace()) {
            self.pos += 1;
        }
    }

    pub fn next_token(&mut self) -> Result<TokenHolder, LexError> {
        self.skip_whitespace();

        if let Some((token, length)) = Token::parse(&self.text[self.pos..]) {
            let ret = TokenHolder::new(token, self.pos, length);
            self.pos += length;
            Ok(ret)
        } else {
            Err(LexError {
                at: self.pos,
                found: self.text[self.pos],
            })
        }
    }
}



#[derive(Debug)]
pub struct LexError {
    at: usize,
    found: char,
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "lexer error at: {}. unexpected character: {}",
            self.at, self.found
        )
    }
}

impl Error for LexError {}

