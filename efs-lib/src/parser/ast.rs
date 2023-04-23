use std::{path::PathBuf, collections::HashMap};

use super::token::{Type, ValueType};



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

pub struct CodeBlock(Vec<CodeLine>);

pub enum CodeLine {
    For(Statement, Statement, Statement, CodeBlock),
    ForList(String, String, CodeBlock),
    StaticFor(String, i32, i32, i32, CodeBlock),
    While(Statement, CodeBlock),
}

pub enum Statement {
    VarDec
}

