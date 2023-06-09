use std::{collections::HashMap, path::PathBuf};

use crate::parser::token::{Keyword, TokenType};

use super::{
    token::{ControlCharacter, Operator, Token, TokenHolder},
    types::{EFSType, EFSValueType},
    ParseError, ParserItem,
};

pub struct Program(pub Vec<Declaration>);

impl ParserItem for Program {
    fn parse(start: usize, tokens: &[TokenHolder]) -> Result<(Self, usize), ParseError> {
        let mut decs = Vec::new();
        let mut pos = start;

        while pos < tokens.len() {
            let (dec, len) = Declaration::parse(pos, &tokens[pos..])?;
            decs.push(dec);
            pos += len;
        }

        Ok((Self(decs), pos))
    }
}

pub enum Declaration {
    FunctionDec {
        is_static: bool,
        attributes: HashMap<String, Vec<String>>,
        name: String,
        parameters: Vec<(String, EFSType)>,
        return_type: EFSType,
        code_block: CodeBlock,
    },
    ConstDec(Assignment),
    UseFile(PathBuf),
    StructDef(String, HashMap<String, EFSType>),
}
impl ParserItem for Declaration {
    fn parse(start: usize, tokens: &[TokenHolder]) -> Result<(Self, usize), ParseError> {
        let first = tokens.first();
        match first.map(|t| t.token.token_type()) {
            Some(TokenType::Keyword(Keyword::Function))
            | Some(TokenType::Keyword(Keyword::Static))
            | Some(TokenType::ControlCharacter(ControlCharacter::Attribute)) => {
                todo!()
            }
            Some(TokenType::Keyword(Keyword::Const)) => {
                let (assignment, len) = Assignment::parse(start + 1, tokens)?;
                Ok((Self::ConstDec(assignment), len + 1))
            }
            Some(TokenType::Keyword(Keyword::UseFile)) => {
                if let Some(token) = tokens.get(1) {
                    if let Token::String(s) = token.token.clone() {
                        Ok((Declaration::UseFile(s.into()), 2))
                    } else {
                        Err(ParseError {
                            at: token.clone(),
                            expected: Vec::from([TokenType::String]),
                        })
                    }
                } else {
                    Err(ParseError {
                        at: TokenHolder {
                            start: first.unwrap().start + first.unwrap().length,
                            length: 0,
                            token: Token::EOI,
                        },
                        expected: Vec::from([TokenType::String]),
                    })
                }
            }
            Some(TokenType::Keyword(Keyword::Struct)) => {
                if let Some(ident_holder) = tokens.get(start + 1) {
                    if let Token::Identifier(ident) = &ident_holder.token {
                        TokenHolder::is_or_eoi(
                            tokens.get(start + 2),
                            TokenType::ControlCharacter(ControlCharacter::LBrace),
                            ident_holder.start + ident_holder.length,
                        )?;

                        let mut pos = 3;
                        loop {
                            if let Token::Identifier(ident) = TokenHolder::is_or_eoi(tokens.get(pos), TokenType::Identifier, pos) {
                                
                            }
                        }

                    }
                }

                todo!()
            }
            _ => Err(ParseError {
                at: tokens
                    .first()
                    .unwrap_or(&TokenHolder {
                        start: start,
                        length: 0,
                        token: Token::EOI,
                    })
                    .clone(),
                expected: Vec::from([
                    TokenType::Keyword(Keyword::Function),
                    TokenType::Keyword(Keyword::Static),
                    TokenType::ControlCharacter(ControlCharacter::Attribute),
                    TokenType::Keyword(Keyword::Const),
                    TokenType::Keyword(Keyword::UseFile),
                    TokenType::Keyword(Keyword::Struct),
                ]),
            }),
        }
    }
}

pub struct Assignment {
    pub name: String,
    pub var_type: Option<EFSType>,
    pub var: Value,
}

impl ParserItem for Assignment {
    fn parse(start: usize, tokens: &[TokenHolder]) -> Result<(Self, usize), ParseError> {
        todo!()
    }
}

pub struct CodeBlock(Vec<Statement>);

pub enum Statement {
    For(Box<Statement>, Box<Statement>, Box<Statement>, CodeBlock),
    ForList(String, String, CodeBlock),
    StaticFor(String, i32, i32, i32, CodeBlock),
    While(Box<Statement>, CodeBlock),
    If(Box<Statement>, CodeBlock),
    Expression(Expression),
}

pub enum Expression {
    VarDec(String, Option<EFSType>, EFSValueType),
    Assign(String, EFSValueType),
}

pub enum Value {
    Value(EFSValueType),
    Math(Box<Math>),
    Identifier(String),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Struct(HashMap<String, Value>),
}

pub struct Math {
    pub left: Value,
    pub op: Operator,
    pub right: Value,
}
