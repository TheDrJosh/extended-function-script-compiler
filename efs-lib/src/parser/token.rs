use std::fmt::Display;

use strum::{Display, EnumIter, IntoEnumIterator};

pub struct TokenHolder {
    pub start: usize,
    pub token: Token,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Integer(i64),
    Float(f64),
    KeyWord(Keyword),
    ControlCharacter(ControlCharacter),
    Operator(Operator),
}

#[derive(Debug, EnumIter, Display)]
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

#[derive(Debug, EnumIter, Display)]
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

#[derive(Debug, EnumIter, Display)]
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

impl LexerType for Operator {
    
}

trait LexerType: Sized + IntoEnumIterator + Display {
    fn parse(text: Vec<char>) -> Option<Self> {
        let mut ret = None;
        for enum_var in Self::iter() {
            if enum_var.to_string().chars().collect::<Vec<char>>() == text[..enum_var.to_string().chars().count()] {
                if enum_var.to_string().chars().count() > ret.map(|ev: Self| ev.to_string().chars().count()).unwrap_or_default() {
                    ret = Some(enum_var);
                }
            }
        }
        ret
    }
}