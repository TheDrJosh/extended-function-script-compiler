use std::{path::PathBuf, collections::HashMap};

use super::types::{EFSType, EFSValueType};


pub struct Program(pub Vec<Declaration>);

pub enum Declaration {
    FunctionDec {
        is_static: bool,
        attributes: HashMap<String, String>,
        name: String,
        parameters: Vec<(String, EFSType)>,
        return_type: EFSType,
        code_block: CodeBlock,
    },
    ConstDec(String, EFSValueType),
    UseFile(PathBuf),
    StructDec(String, HashMap<String, EFSType>),
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
    AnyType(String, ),
}

