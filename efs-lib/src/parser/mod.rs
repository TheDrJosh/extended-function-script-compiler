use std::path::PathBuf;

use self::{
    ast::{Declaration, Program, Value},
    lexer::Lexer,
    token::{ControlCharacter, Keyword, Token, TokenHolder, TokenType},
    types::EFSType,
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

    // fn constant_definition(&mut self) -> anyhow::Result<Option<Declaration>> {
    //     let mut ret = None;

    //     if self.current_token.token.token_type() == TokenType::Keyword(Keyword::Const) {
    //         self.eat(TokenType::Keyword(Keyword::Const))?;

    //         let (ident, const_type, value) = self.assignment()?;

    //         let const_type = const_type.ok_or(anyhow::anyhow!(
    //             "const declarations require type annotation."
    //         ))?;

    //         ret = Some(Declaration::ConstDec(ident, const_type, value))
    //     }
    //     Ok(ret)
    // }

    // fn assignment(&mut self) -> anyhow::Result<(String, Option<EFSType>, Value)> {
    //     if let Token::Identifier(ident) = self.current_token.token.clone() {
    //         self.eat(TokenType::Identifier)?;

    //         let assign_type = if self.current_token.token.token_type()
    //             == TokenType::ControlCharacter(ControlCharacter::TypeClarify)
    //         {
    //             self.eat(TokenType::ControlCharacter(ControlCharacter::TypeClarify))?;
    //             match self.current_token.token.clone() {
    //                 Token::TypeName(var_type) => {
    //                     self.eat(TokenType::TypeName)?;
    //                     Some(var_type.to_type())
    //                 }
    //                 Token::Identifier(type_ident) => {
    //                     self.eat(TokenType::Identifier)?;
    //                     Some(EFSType::Struct(type_ident))
    //                 }
    //                 _ => Err(self.error(&[TokenType::TypeName, TokenType::Identifier]))?,
    //             }
    //         } else {
    //             None
    //         };
    //         self.eat(TokenType::ControlCharacter(ControlCharacter::Assign))?;

    //         let value = self.value()?;

    //         return Ok((ident, assign_type, value));
    //     }

    //     Err(self.error(&[TokenType::Identifier]))
    // }

    // fn use_file(&mut self) -> anyhow::Result<Option<Declaration>> {
    //     if self.current_token.token == Token::Keyword(Keyword::UseFile) {
    //         self.eat(TokenType::Keyword(Keyword::UseFile))?;
    //         let path: PathBuf = if let Token::String(path_str) = &self.current_token.token {
    //             path_str.into()
    //         } else {
    //             Err(self.error(&[TokenType::String]))?
    //         };
    //         Ok(Some(Declaration::UseFile(path)))
    //     } else {
    //         Ok(None)
    //     }
    // }

}
