use crate::parser::token::Token;

use super::token::TokenHolder;

pub struct Lexer {
    text: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text: text.chars().collect(),
            pos: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.text.get(self.pos).map_or(false, |c| c.is_whitespace()) {
            self.pos += 1;
        }
    }

    pub fn next_token(&mut self) -> Result<TokenHolder, usize> {
        self.skip_whitespace();

        if let Some((token, length)) = Token::parse(&self.text[self.pos..]) {
            let ret = TokenHolder::new(token, self.pos, length);
            self.pos += length;
            Ok(ret)
        } else {
            Err(self.pos)
        }
    }
}
