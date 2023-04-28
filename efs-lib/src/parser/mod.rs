use self::{
    ast::{Program, Declaration},
    lexer::Lexer,
    token::{ControlCharacter, Keyword, Token, TokenHolder, TokenType},
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
        let mut lexer = Lexer::new(text.clone());
        let current_token = lexer.next_token()?;
        Ok(Self {
            text: text,
            lexer,
            current_token,
        })
    }

    fn error(&self, got: &TokenHolder, expected: &[TokenType]) -> anyhow::Error {
        anyhow::anyhow!(
            "Invalid syntax, got: {:?}, expected: {:?}, at {}",
            got.token,
            expected,
            got.start
        )
    }

    fn eat(&mut self, token_type: TokenType) -> anyhow::Result<()> {
        let token = self.lexer.next_token()?;
        if token.token.token_type() == token_type {
            self.current_token = token;
            Ok(())
        } else {
            Err(self.error(&token, &[token_type]))
        }
    }

    pub fn prog(&mut self) -> anyhow::Result<Program> {
        let dec = Vec::new();

        

        Ok(Program(dec))
    }

    fn function_definition(&mut self) -> Option<Declaration>{
        todo!()
    }


}
