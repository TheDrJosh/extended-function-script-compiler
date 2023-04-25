use std::fmt::Display;

use strum::{Display, EnumIter, IntoEnumIterator};

pub struct TokenHolder {
    pub start: usize,
    pub token: Token,
}

impl TokenHolder {
    pub fn new(token: Token, start: usize) -> Self {
        Self { start, token }
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

impl Token {
    pub fn parse(text: Vec<char>) -> Option<(Self, usize)> {
        if text.is_empty() {
            return Some((Self::EOI, 0));
        }

        let results = [
            Keyword::parse(text.clone()).map(|res| (Token::KeyWord(res.0), res.1)),
            ControlCharacter::parse(text.clone())
                .map(|res| (Token::ControlCharacter(res.0), res.1)),
            Operator::parse(text.clone()).map(|res| (Token::Operator(res.0), res.1)),
            Self::parse_identifier(text.clone()),
            Self::parse_number(text.clone()),
        ];

        results.iter().max_by(|&res1, &res2| {
            res1.as_ref()
                .map(|res| res.1)
                .unwrap_or_default()
                .cmp(&res2.as_ref().map(|res| res.1).unwrap_or_default())
        })?.clone()
    }

    fn parse_identifier(text: Vec<char>) -> Option<(Self, usize)> {
        let mut ident = String::default();

        if text.get(0).map(|c| c.is_alphabetic())? {
            ident.push(*text.get(0)?);

            let mut i = 1;
            while text.get(i).map_or(false, |c| c.is_alphanumeric()) {
                ident.push(*text.get(i)?);

                i += 1;
            }
        }
        if ident.is_empty() {
            return None;
        }

        Some((Self::Identifier(ident.clone()), ident.chars().count()))
    }

    fn parse_number(text: Vec<char>) -> Option<(Self, usize)> {
        let mut result = String::default();

        let mut pos = 0;

        if text.get(pos)? == &'-' {
            result.push('-');
            pos += 1;
        }

        while text.get(pos).map_or(false, |char| char.is_ascii_digit()) {
            result.push(*text.get(pos)?);
            pos += 1;
        }

        if text.get(pos).map_or(false, |char| char == &'.') {
            result.push('.');
            pos += 1;

            while text.get(pos).map_or(false, |char| char.is_ascii_digit()) {
                result.push(*text.get(pos)?);
                pos += 1;
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

impl LexerType for Operator {}

trait LexerType: Sized + IntoEnumIterator + Display + Clone {
    fn parse(text: Vec<char>) -> Option<(Self, usize)> {
        let mut ret: Option<(Self, usize)> = None;
        for enum_var in Self::iter() {
            if enum_var.to_string().chars().collect::<Vec<char>>()
                == text[..enum_var.to_string().chars().count()]
            {
                if enum_var.to_string().chars().count() > ret.clone().map(|ev| ev.1).unwrap_or_default() {
                    ret = Some((enum_var.clone(), enum_var.to_string().chars().count()));
                }
            }
        }
        ret
    }
}
