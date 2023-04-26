use std::fmt::Display;

use strum::{Display, EnumIter, IntoEnumIterator};

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
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Integer(i64),
    Float(f64),
    KeyWord(Keyword),
    ControlCharacter(ControlCharacter),
    Operator(Operator),
    EOI,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Integer,
    Float,
    KeyWord,
    ControlCharacter,
    Operator,
    EOI,
}

impl Token {
    pub fn token_type(&self) -> TokenType {
        match self {
            Token::Identifier(_) => TokenType::Identifier,
            Token::Integer(_) => TokenType::Integer,
            Token::Float(_) => TokenType::Float,
            Token::KeyWord(_) => TokenType::KeyWord,
            Token::ControlCharacter(_) => TokenType::ControlCharacter,
            Token::Operator(_) => TokenType::Operator,
            Token::EOI => TokenType::EOI,
        }
    }
}

impl Token {
    pub fn parse(text: &[char]) -> Option<(Self, usize)> {
        //println!("Parsing: {:?}", text);

        if text.is_empty() {
            return Some((Self::EOI, 0));
        }

        let results = [
            Self::parse_identifier(text),
            Self::parse_number(text),
            Keyword::parse(text).map(|res| (Token::KeyWord(res.0), res.1)),
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

    fn parse_identifier(text: &[char]) -> Option<(Self, usize)> {
        let mut ident = String::default();
        let mut pos = 0;

        if text.get(pos).map(|c| c.is_alphabetic())? {
            ident.push(text.get(pos)?.clone());
            pos += 1;

            while text.get(pos).map_or(false, |c| c.is_alphanumeric()) {
                ident.push(text.get(pos)?.clone());
                pos += 1;
            }
        }
        if ident.is_empty() {
            return None;
        }

        Some((Self::Identifier(ident.clone()), ident.chars().count()))
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

#[derive(Debug, EnumIter, Display, Clone)]
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

#[derive(Debug, EnumIter, Display, Clone)]
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
    LeftParenthesis,
    #[strum(serialize = ")")]
    RightParenthesis,
    #[strum(serialize = "[")]
    LBracket,
    #[strum(serialize = "]")]
    RBracket,
    #[strum(serialize = "{")]
    LBrace,
    #[strum(serialize = "}")]
    RBrace,
}

impl LexerType for ControlCharacter {}

#[derive(Debug, EnumIter, Display, Clone)]
pub enum Operator {
    #[strum(serialize = "=")]
    Assign,
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
            Operator::Assign => 0,
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

trait LexerType: Sized + IntoEnumIterator + Display + Clone {
    fn parse(text: &[char]) -> Option<(Self, usize)> {
        Self::iter()
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
