use std::{path::PathBuf, collections::HashMap};

use super::types::{ValueType, Type};




pub struct Program(Vec<ProgramInner>);

pub enum ProgramInner {
    FunctionDec {
        is_static: bool,
        attributes: HashMap<String, ValueType>,
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        code_block: CodeBlock,
    },
    ConstDec(String, ValueType),
    UseFile(PathBuf),
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
    VarDec(String, Option<Type>, ValueType),
    Assign(String, ValueType),
    AnyType(String, ),
}

