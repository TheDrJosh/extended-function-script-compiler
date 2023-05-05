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

    pub fn next_token(&mut self) -> anyhow::Result<TokenHolder> {
        self.skip_whitespace();

        if let Some((token, length)) = Token::parse(&self.text[self.pos..]) {
            let ret = TokenHolder::new(token, self.pos, length);
            self.pos += length;
            Ok(ret)
        } else {
            anyhow::bail!("Unknown Token at: {}", self.pos);
        }
    }
}

pub struct TokenStream {
    history: Vec<TokenHolder>,
    lexer: Lexer,
}

impl TokenStream {
    pub fn from(text: String) -> Self {
        Self {
            history: Vec::new(),
            lexer: Lexer::new(text),
        }
    }
    
}


impl Iterator for TokenStream {
    type Item = TokenHolder;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.next_token();
        self.history.push(token);
        return Some(token)
    }
}
