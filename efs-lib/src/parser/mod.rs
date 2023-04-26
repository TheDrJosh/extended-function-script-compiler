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
    current_token: TokenHolder,
}

impl Parser {
    pub fn new(text: String) -> anyhow::Result<Self> {
        let mut lexer = Lexer::new(text);
        let current_token = lexer.next_token();
        Ok(Self {
            text: text.clone(),
            lexer,
            current_token,
        })
    }

    fn error(&self, got: &TokenHolder, expected: Option<TokenType>) -> anyhow::Error {
        match expected {
            Some(expected) => {
                anyhow::anyhow!("Invalid syntax, got: {:?}, expected: {:?}, at {}", got.token, expected, got.start)
            }
            None => anyhow::anyhow!("", got, expected,),
        }
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

    pub fn prog(&mut self) -> () {}
}
