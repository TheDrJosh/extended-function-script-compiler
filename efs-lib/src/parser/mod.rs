use self::{lexer::Lexer, token::{Token, TokenType}};

pub mod lexer;
pub mod token;
pub mod ast;

pub struct Parser {
    text: String,
    tokens: Vec<Token>,
    current_token: usize,
}

impl Parser {
    pub fn new(text: String) -> anyhow::Result<Self> {
        let mut lexer = Lexer::new(text.clone());
        let tokens = lexer.get_all_tokens()?;
        Ok(Self {
            text,
            tokens,
            current_token: 0,
        })
    }

    fn get(&self, i: usize) -> anyhow::Result<&Token> {
        if let Some(token) = self.tokens.get(i) {
            Ok(token)
        } else {
            anyhow::bail!("token index out of bounds");
        }
    }

    fn current_token(&self) -> anyhow::Result<&Token> {
        self.get(self.current_token)
    }

    fn error(&self, token: &Token, expected: TokenType) -> anyhow::Error {
        let mut arrow = " ".repeat(token.start - 1);
        arrow.push('^');
        anyhow::anyhow!(
            "Invalid syntax, expected {:?}\n{}\n{}",
            expected,
            self.text.trim_end(),
            arrow
        )
    }

    fn eat(&mut self, token_type: TokenType) -> anyhow::Result<()> {
        if self.current_token()?.typ == token_type {
            self.current_token += 1;
        } else {
            Err(self.error(self.current_token()?, token_type))?;
        }
        Ok(())
    }

    fn prog(&mut self) -> () {
        
    }



}