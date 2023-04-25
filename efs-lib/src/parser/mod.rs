use self::{
    lexer::Lexer,
    token::{Token, TokenHolder, TokenType},
};

pub mod ast;
pub mod lexer;
pub mod token;
pub mod types;

pub struct Parser {
    text: String,
    lexer: Lexer,
    current_token: Option<TokenHolder>,
}

impl Parser {
    pub fn new(text: String) -> anyhow::Result<Self> {
        Ok(Self {
            text: text.clone(),
            lexer: Lexer::new(text),
            current_token: None,
        })
    }

    fn error(&self, got: &TokenHolder, expected: TokenType) -> anyhow::Error {
        anyhow::anyhow!("Invalid syntax, got: {:?}, expected: {:?}", got, expected,)
    }

    fn eat(&mut self, token_type: TokenType) -> anyhow::Result<()> {
        match self.lexer.next_token() {
            Ok(token) => {
                if token.token.token_type() == token_type {
                    self.current_token = Some(token);
                    Ok(())
                } else {
                    Err(self.error(&token, token_type))
                }
            }
            Err(err_pos) => anyhow::bail!("Error: unknown token at {}", err_pos),
        }
    }

        fn prog(&mut self) -> () {

        }
}
