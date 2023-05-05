
use self::{
    lexer::Lexer,
    token::{TokenHolder, TokenType}
};

pub mod ast;
pub mod lexer;
pub mod token;
pub mod types;

pub trait ParserItem where Self: Sized {
    fn parse(start: usize, tokens: &[TokenHolder]) -> Result<(Self, usize), ParseError>;
}

pub struct ParseError {
    pub at: TokenHolder,
    pub expected: Vec<TokenType>,
}

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

    fn error(&self, expected: &[TokenType]) -> anyhow::Error {
        anyhow::anyhow!(
            "Invalid syntax, got: {:?}, expected: {:?}, at {}",
            self.current_token.token,
            expected,
            self.current_token.start
        )
    }


}
