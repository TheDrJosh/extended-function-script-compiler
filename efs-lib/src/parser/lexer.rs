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

    pub fn next_token(&mut self) -> anyhow::Result<TokenHolder> {
        if let Some((token, length)) = Token::parse(self.text[self.pos..].to_vec()) {
            let ret = TokenHolder::new(token, self.pos);
            self.pos += length;
            Ok(ret)
        } else {
            anyhow::bail!("Error near {}", self.pos);
        }
    }
}
