use std::{fmt::Display, path::PathBuf};

use enum_iterator::{all, Sequence};
use strum::Display;

use super::{types::EFSType, ParseError};





#[derive(Debug, Clone)]
pub struct TokenHolder {
    pub start: usize,
    pub length: usize,
    pub token: Token,
}

impl TokenHolder {
    pub fn new(token: Token, start: usize, length: usize) -> Self {
        Self {
            start,
            length,
            token,
        }
    }
    pub fn is(&self, expected: TokenType) -> Result<&Self, ParseError> {
        if self.token.is(expected.clone()) {
            Ok(self)
        } else {
            Err(ParseError {
                at: self.clone(),
                expected: Vec::from([expected]),
            })
        }
    }
    pub fn is_many(&self, expected: &[TokenType]) -> Result<&Self, ParseError> {
        if self.token.is_many(expected.clone()) {
            Ok(self)
        } else {
            Err(ParseError {
                at: self.clone(),
                expected: Vec::from(expected),
            })
        }
    }
    pub fn is_or_eoi(
        s: Option<&Self>,
        expected: TokenType,
        pos: usize,
    ) -> Result<&Self, ParseError> {
        if let Some(s) = s {
            s.is(expected)
        } else {
            Err(ParseError {
                at: TokenHolder {
                    start: pos,
                    length: 0,
                    token: Token::EOI,
                },
                expected: Vec::from([expected]),
            })
        }
    }
    pub fn is_many_or_eoi<'a>(
        s: Option<&'a Self>,
        expected: &[TokenType],
        pos: usize,
    ) -> Result<&'a Self, ParseError> {
        if let Some(s) = s {
            s.is_many(expected)
        } else {
            Err(ParseError {
                at: TokenHolder {
                    start: pos,
                    length: 0,
                    token: Token::EOI,
                },
                expected: Vec::from(expected),
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Keyword(Keyword),
    ControlCharacter(ControlCharacter),
    Operator(Operator),
    FilePath(PathBuf),
    EOI,
    TypeName(TypeName),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Integer,
    Float,
    String,
    Keyword(Keyword),
    ControlCharacter(ControlCharacter),
    Operator(Operator),
    FilePath,
    EOI,
    TypeName,
}

impl Token {
    pub fn token_type(&self) -> TokenType {
        match self {
            Token::Identifier(_) => TokenType::Identifier,
            Token::Integer(_) => TokenType::Integer,
            Token::Float(_) => TokenType::Float,
            Token::Keyword(kw) => TokenType::Keyword(kw.clone()),
            Token::ControlCharacter(cc) => TokenType::ControlCharacter(cc.clone()),
            Token::Operator(op) => TokenType::Operator(op.clone()),
            Token::EOI => TokenType::EOI,
            Token::String(_) => TokenType::String,
            Token::FilePath(_) => TokenType::FilePath,
            Token::TypeName(_) => TokenType::TypeName,
        }
    }
    pub fn is(&self, expected: TokenType) -> bool {
        self.token_type() == expected
    }
    pub fn is_many(&self, expected: &[TokenType]) -> bool {
        expected
            .iter()
            .map(|e| e == &self.token_type())
            .reduce(|a, b| a | b)
            .unwrap_or_default()
    }
}

impl Token {
    pub fn parse(text: &[char]) -> Option<(Self, usize)> {
        //println!("Parsing: {:?}", text);

        if text.is_empty() {
            return Some((Self::EOI, 0));
        }

        let results = [
            Self::parse_path(text),
            Self::parse_identifier(text),
            Self::parse_number(text),
            Self::parse_string(text),
            Keyword::parse(text).map(|res| (Token::Keyword(res.0), res.1)),
            ControlCharacter::parse(text).map(|res| (Token::ControlCharacter(res.0), res.1)),
            Operator::parse(text).map(|res| (Token::Operator(res.0), res.1)),
        ];

        results
            .iter()
            .max_by(|&res1, &res2| {
                res1.as_ref()
                    .map(|res| res.1)
                    .unwrap_or_default()
                    .cmp(&res2.as_ref().map(|res| res.1).unwrap_or_default())
            })?
            .clone()
    }

    fn parse_path(text: &[char]) -> Option<(Self, usize)> {
        let mut path = String::default();
        let mut pos = 0;

        while text.get(pos).map_or(false, |c| {
            c.is_alphanumeric() || c == &'/' || c == &'.' || c == &'_'
        }) {
            path.push(text.get(pos)?.clone());
            pos += 1;
        }

        if path.is_empty() {
            return None;
        }

        Some((Self::FilePath(path.clone().into()), path.chars().count()))
    }

    fn parse_identifier(text: &[char]) -> Option<(Self, usize)> {
        let mut ident = String::default();
        let mut pos = 0;

        if text.get(pos).map(|c| c.is_alphabetic() || c == &'_')? {
            ident.push(text.get(pos)?.clone());
            pos += 1;

            while text
                .get(pos)
                .map_or(false, |c| c.is_alphanumeric() || c == &'_')
            {
                ident.push(text.get(pos)?.clone());
                pos += 1;
            }
        }
        if ident.is_empty() {
            return None;
        }

        Some((Self::Identifier(ident.clone()), ident.chars().count()))
    }
    fn parse_string(text: &[char]) -> Option<(Self, usize)> {
        let mut string = String::default();
        let mut pos = 0;

        if text.get(pos) == Some(&'"') {
            pos += 1;

            while text.get(pos) != Some(&'"') {
                string.push(text.get(pos)?.clone());
                pos += 1;
            }

            Some((Self::String(string.clone()), string.chars().count() + 2))
        } else {
            None
        }
    }

    fn parse_number(text: &[char]) -> Option<(Self, usize)> {
        let mut result = String::default();

        let mut pos = 0;

        if text.get(pos)? == &'-' {
            result.push('-');
            pos += 1;
        }

        while text.get(pos).map_or(false, |char| char.is_ascii_digit()) {
            result.push(text.get(pos)?.clone());
            pos += 1;
        }

        if text.get(pos).map_or(false, |char| char == &'.') {
            result.push('.');
            pos += 1;

            while text.last().map_or(false, |char| char.is_ascii_digit()) {
                result.push(text.get(pos)?.clone());
            }

            Some((Token::Float(result.parse().ok()?), result.chars().count()))
        } else {
            Some((Token::Integer(result.parse().ok()?), result.chars().count()))
        }
    }
}

#[derive(Debug, Sequence, Clone, PartialEq, Display)]
pub enum Keyword {
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "fn")]
    Function,
    #[strum(serialize = "for")]
    For,
    #[strum(serialize = "while")]
    While,
    #[strum(serialize = "const")]
    Const,
    #[strum(serialize = "let")]
    VarDeceleration,
    #[strum(serialize = "use")]
    UseFile,
    #[strum(serialize = "if")]
    If,
    #[strum(serialize = "struct")]
    Struct,
    #[strum(serialize = "in")]
    In,
    #[strum(serialize = "true")]
    True,
    #[strum(serialize = "false")]
    False,
    #[strum(serialize = "None")]
    None,
}

impl LexerType for Keyword {}

#[derive(Debug, Sequence, Display, Clone, PartialEq)]
pub enum TypeName {
    #[strum(serialize = "num")]
    Number,
    #[strum(serialize = "i8")]
    Byte,
    #[strum(serialize = "i16")]
    Short,
    #[strum(serialize = "i32")]
    Int,
    #[strum(serialize = "i64")]
    Long,
    #[strum(serialize = "f32")]
    Float,
    #[strum(serialize = "f64")]
    Double,
    #[strum(serialize = "string")]
    String,
    #[strum(serialize = "bool")]
    Bool,
    // #[strum(to_string = "List<{0}>")]
    // List(Box<EFSType>),
    #[strum(serialize = "Dict")]
    Dict,
    #[strum(serialize = "NBTi8Array")]
    NBTByteArray,
    #[strum(serialize = "NBTi32Array")]
    NBTIntArray,
    #[strum(serialize = "NBTi64Array")]
    NBTLongArray,
    #[strum(serialize = "_")]
    None,
}

impl TypeName {
    pub fn to_type(&self) -> EFSType {
        match self {
            TypeName::Number => EFSType::Number,
            TypeName::Byte => EFSType::Byte,
            TypeName::Short => EFSType::Short,
            TypeName::Int => EFSType::Int,
            TypeName::Long => EFSType::Long,
            TypeName::Float => EFSType::Float,
            TypeName::Double => EFSType::Double,
            TypeName::String => EFSType::String,
            TypeName::Bool => EFSType::Bool,
            TypeName::Dict => EFSType::Dict,
            TypeName::NBTByteArray => EFSType::NBTByteArray,
            TypeName::NBTIntArray => EFSType::NBTIntArray,
            TypeName::NBTLongArray => EFSType::NBTLongArray,
            TypeName::None => EFSType::None,
        }
    }
}

#[derive(Debug, Sequence, Display, Clone, PartialEq)]
pub enum ControlCharacter {
    #[strum(serialize = ";")]
    EndOfLine,
    #[strum(serialize = "#")]
    Attribute,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = "->")]
    FunctionReturn,
    #[strum(serialize = ":")]
    TypeClarify,
    #[strum(serialize = ".")]
    Dot,
    #[strum(serialize = "::")]
    Scope,
    #[strum(serialize = "(")]
    LeftParen,
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = "[")]
    LBracket,
    #[strum(serialize = "]")]
    RBracket,
    #[strum(serialize = "{")]
    LBrace,
    #[strum(serialize = "}")]
    RBrace,
    #[strum(serialize = "=")]
    Assign,
}

impl LexerType for ControlCharacter {}

#[derive(Debug, Sequence, Display, Clone, PartialEq)]
pub enum Operator {
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "*")]
    Multi,
    #[strum(serialize = "/")]
    Div,
    #[strum(serialize = "==")]
    Equal,
    #[strum(serialize = "!=")]
    NotEqual,
    #[strum(serialize = "<")]
    Less,
    #[strum(serialize = "<=")]
    LessOrEqual,
    #[strum(serialize = ">")]
    Greater,
    #[strum(serialize = ">=")]
    GreaterOrEqual,
    #[strum(serialize = "||")]
    LogicalOr,
    #[strum(serialize = "&&")]
    LogicalAnd,
    #[strum(serialize = "|")]
    Or,
    #[strum(serialize = "&")]
    And,
    #[strum(serialize = "^")]
    Xor,
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Plus => 6,
            Operator::Minus => 6,
            Operator::Multi => 5,
            Operator::Div => 5,
            Operator::Equal => 2,
            Operator::NotEqual => 2,
            Operator::Less => 2,
            Operator::LessOrEqual => 2,
            Operator::Greater => 2,
            Operator::GreaterOrEqual => 2,
            Operator::LogicalOr => 3,
            Operator::LogicalAnd => 3,
            Operator::Or => 4,
            Operator::And => 4,
            Operator::Xor => 4,
        }
    }
}

impl LexerType for Operator {}

pub trait LexerType: Sized + Display + Clone + Sequence {
    fn parse(text: &[char]) -> Option<(Self, usize)> {
        all::<Self>()
            .filter_map(|x| {
                let name = x.to_string();
                for (i, c) in name.chars().enumerate() {
                    if text.get(i) != Some(&c) {
                        return None;
                    }
                }
                Some((x, name.chars().count()))
            })
            .max_by(|x, y| x.1.cmp(&y.1))
            .filter(|x| x.1 != 0)
    }
}
