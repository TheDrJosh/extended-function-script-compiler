use crate::parser::token::Token;

use super::token::TokenHolder;

pub struct Lexer {
    text: Vec<char>,
    length: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text: text.chars().rev().collect(),
            length: text.chars().count(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.text.last().map_or(false, |c| c.is_whitespace()) {
            self.text.pop();
        }
    }

    pub fn next_token(&mut self) -> Result<TokenHolder, usize> {
        self.skip_whitespace();

        if let Some((token, length)) = Token::parse(&mut self.text) {
            let ret = TokenHolder::new(token, self.length - self.text.len());
            Ok(ret)
        } else {
            Err(self.length - self.text.len())
        }
    }
}
