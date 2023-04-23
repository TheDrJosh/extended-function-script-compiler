use crate::parser::token::{Keyword, TokenType};

use super::token::Token;

pub struct Lexer {
    text: Vec<char>,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let current_char = text.chars().next().clone();
        Self {
            text: text.chars().collect(),
            pos: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;

        if self.pos > self.text.len() - 1 {
            self.current_char = None;
        } else {
            self.current_char = Some(self.text[self.pos]);
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.map_or(false, |char| char.is_whitespace()) {
            self.advance();
        }
    }

    fn number(&mut self) -> Token {
        let mut result = String::default();
        let pos = self.pos;

        if self.current_char == Some('-') {
            result.push('-');
            self.advance();
        }

        while self
            .current_char
            .map_or(false, |char| char.is_ascii_digit())
        {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        if self.current_char == Some('.') {
            result.push('.');
            self.advance();

            while self
                .current_char
                .map_or(false, |char| char.is_ascii_digit())
            {
                result.push(self.current_char.unwrap());
                self.advance();
            }

            Token::new(result, TokenType::Float, pos)
        } else {
            Token::new(result, TokenType::Integer, pos)
        }
    }

    fn identifier(&mut self) -> Token {
        let mut result = String::default();
        let pos = self.pos;

        while self.current_char.map_or(false, |char| char.is_alphabetic()) {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        Token::new(result, TokenType::Identifier, pos)
    }

    fn key_word(&mut self) -> Option<Token> {
        const KEYWORD: [(&str, Keyword); 8] = [
            ("static ", Keyword::Static),
            ("fn ", Keyword::Function),
            ("for", Keyword::For),
            ("while", Keyword::While),
            ("const ", Keyword::Const),
            ("let ", Keyword::VarDeceleration),
            ("use ", Keyword::UseFile),
            ("if ", Keyword::If),
        ];
        let pos = self.pos;
        for key_word in KEYWORD {
            if let Some(text) = self
                .text
                .get(self.pos..self.pos + key_word.0.chars().count())
            {
                let mut equal = true;
                for (i, c) in key_word.0.char_indices() {
                    equal &= text.get(i) == Some(&c);
                }
                if equal {
                    for _ in key_word.0.chars() {
                        self.advance();
                    }
                    return Some(Token::new(
                        key_word.0.to_owned(),
                        TokenType::KeyWord(key_word.1),
                        pos,
                    ));
                }
            }
        }
        None
    }

    fn operators(&mut self) -> Option<Token> {
        let res = match self.current_char {
            Some('#') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("#"), TokenType::Attribute, pos))
            }
            Some(',') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from(","), TokenType::Comma, pos))
            }
            Some(';') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from(";"), TokenType::EndLine, pos))
            }
            Some(':') => {
                let pos: usize = self.pos;
                self.advance();
                if self.current_char == Some(':') {
                    self.advance();
                    Some(Token::new(String::from("::"), TokenType::Scope, pos))
                } else {
                    Some(Token::new(String::from(":"), TokenType::TypeClarify, pos))
                }
            }
            Some('=') => {
                let pos: usize = self.pos;
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Some(Token::new(String::from("=="), TokenType::Equal, pos))
                } else {
                    Some(Token::new(String::from("="), TokenType::Assign, pos))
                }
            }
            Some('+') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("+"), TokenType::Plus, pos))
            }
            Some('-') => {
                let pos = self.pos;
                self.advance();
                if self.current_char == Some('>') {
                    self.advance();
                    Some(Token::new(String::from("->"), TokenType::Equal, pos))
                } else {
                    Some(Token::new(String::from("-"), TokenType::Minus, pos))
                }
            }
            Some('*') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("*"), TokenType::Multi, pos))
            }
            Some('/') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("/"), TokenType::Div, pos))
            }
            Some('(') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("("), TokenType::LParen, pos))
            }
            Some(')') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from(")"), TokenType::RParen, pos))
            }
            Some('[') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("["), TokenType::LBracket, pos))
            }
            Some(']') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("]"), TokenType::RBracket, pos))
            }
            Some('{') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("{"), TokenType::LBraces, pos))
            }
            Some('}') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("}"), TokenType::RBraces, pos))
            }
            Some('.') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("."), TokenType::Dot, pos))
            }
            Some('<') => {
                let pos: usize = self.pos;
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Some(Token::new(String::from("<="), TokenType::LessOrEqual, pos))
                } else {
                    Some(Token::new(String::from("<"), TokenType::Less, pos))
                }
            }
            Some('>') => {
                let pos: usize = self.pos;
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Some(Token::new(
                        String::from(">="),
                        TokenType::GreaterOrEqual,
                        pos,
                    ))
                } else {
                    Some(Token::new(String::from(">"), TokenType::Greater, pos))
                }
            }
            Some('!') => {
                let pos: usize = self.pos;
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Some(Token::new(String::from("!="), TokenType::NotEqual, pos))
                } else {
                    Some(Token::new(String::from("!"), TokenType::Not, pos))
                }
            }
            Some('|') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("|"), TokenType::Or, pos))
            }
            Some('&') => {
                let pos = self.pos;
                self.advance();
                Some(Token::new(String::from("&"), TokenType::And, pos))
            }
            _ => None,
        };
        res
    }

    pub fn get_next_token(&mut self) -> anyhow::Result<Token> {
        loop {
            if let Some(current_char) = self.current_char {
                if current_char.is_whitespace() {
                    self.skip_whitespace();
                    continue;
                }

                if current_char.is_alphabetic() {
                    if let Some(token) = self.key_word() {
                        return Ok(token);
                    } else {
                        return Ok(self.identifier());
                    }
                }

                if current_char.is_ascii_digit() || current_char == '-' {
                    return Ok(self.number());
                }

                if let Some(token) = self.operators() {
                    return Ok(token);
                }

                anyhow::bail!("Invalid character: {}", current_char);
            } else {
                break;
            }
        }
        Ok(Token::new(
            "".to_owned(),
            TokenType::EOI,
            self.text.len() - 1,
        ))
    }

    pub fn get_all_tokens(&mut self) -> anyhow::Result<Vec<Token>> {
        let mut res = Vec::new();
        loop {
            let token = self.get_next_token()?;
            res.push(token.clone());
            if token.typ == TokenType::EOI {
                break;
            }
        }
        Ok(res)
    }
}
