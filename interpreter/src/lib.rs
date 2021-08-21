#![allow(dead_code, unused_variables)]

mod ast;
mod lexer;
mod parser;
mod pos;

pub use crate::pos::{Pos, Span};

#[derive(Debug)]
pub struct Ast {
    program: ast::Program,
}

pub fn parse(source: &str) -> Result<Ast, Error> {
    Ok(Ast {
        program: crate::parser::parse(source)?,
    })
}

#[derive(Debug)]
pub struct Error {
    pub span: Span,
    pub message: String,
}
