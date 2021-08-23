mod ast;
mod eval;
mod lexer;
mod parser;
mod pos;

use std::{io::Write, rc::Rc};

pub use crate::pos::{Pos, Span};

#[derive(Debug)]
pub struct Ast {
    source: Rc<Source>,
    program: ast::Program,
}

pub fn parse(source: &str) -> Result<Ast, Error> {
    Ok(Ast {
        source: Rc::new(Source::new(source.to_owned())),
        program: crate::parser::parse(source)?,
    })
}

pub fn eval(ast: &Ast, output: Box<dyn Write>) -> Result<(), Error> {
    let mut evaluator = eval::Evaluator::new(
        ast.source.clone(),
        Some(&ast.program),
        output,
    );
    evaluator.run_program(&ast.program)
        .map_err(|e| Error { span: e.span.unwrap(), message: e.message })
}

pub fn tokens(source: &str) -> impl Iterator<Item = Token> + '_ {
    crate::parser::tokens(source)
        .filter_map(|(kind, span)| TokenKind::from_lexer(kind)
            .map(|kind| Token { kind, span }))
}

#[derive(Debug)]
pub struct Error {
    pub span: Span,
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Keyword,
    Value,
    String,
    Operator,
    Punctuation,
    Number,
    Comment,
}

impl TokenKind {
    fn from_lexer(kind: crate::lexer::TokenKind) -> Option<Self> {
        match kind {
            crate::lexer::TokenKind::Fn |
            crate::lexer::TokenKind::Let |
            crate::lexer::TokenKind::While |
            crate::lexer::TokenKind::If |
            crate::lexer::TokenKind::Else |
            crate::lexer::TokenKind::For |
            crate::lexer::TokenKind::In |
            crate::lexer::TokenKind::Return |
            crate::lexer::TokenKind::Struct |
            crate::lexer::TokenKind::True |
            crate::lexer::TokenKind::False |
            crate::lexer::TokenKind::Nil |
            crate::lexer::TokenKind::SelfKw => Some(TokenKind::Keyword),
            crate::lexer::TokenKind::Str => Some(TokenKind::String),
            crate::lexer::TokenKind::Dot |
            crate::lexer::TokenKind::Colon => Some(TokenKind::Punctuation),
            crate::lexer::TokenKind::Equals |
            crate::lexer::TokenKind::And |
            crate::lexer::TokenKind::Or |
            crate::lexer::TokenKind::Plus |
            crate::lexer::TokenKind::Minus |
            crate::lexer::TokenKind::Star |
            crate::lexer::TokenKind::Slash |
            crate::lexer::TokenKind::Less |
            crate::lexer::TokenKind::LessEq |
            crate::lexer::TokenKind::Greater |
            crate::lexer::TokenKind::GreaterEq |
            crate::lexer::TokenKind::EqEq |
            crate::lexer::TokenKind::NotEq => Some(TokenKind::Operator),
            crate::lexer::TokenKind::LeftParen |
            crate::lexer::TokenKind::RightParen |
            crate::lexer::TokenKind::LeftCurly |
            crate::lexer::TokenKind::RightCurly |
            crate::lexer::TokenKind::Comma |
            crate::lexer::TokenKind::Semicolon => Some(TokenKind::Punctuation),
            crate::lexer::TokenKind::Name => Some(TokenKind::Value),
            crate::lexer::TokenKind::Number => Some(TokenKind::Number),
            crate::lexer::TokenKind::CommentMarker |
            crate::lexer::TokenKind::CodeMarker => Some(TokenKind::Comment),
            crate::lexer::TokenKind::Space |
            crate::lexer::TokenKind::Newline |
            crate::lexer::TokenKind::Error => None,
            crate::lexer::TokenKind::CommentText => Some(TokenKind::Comment),
        }
    }
}

#[derive(Debug)]
struct Source {
    text: String,
    line_starts: Vec<usize>,
}

impl Source {
    fn new(source: String) -> Source {
        let mut line_starts = vec![0];
        line_starts.extend(source
            .char_indices()
            .filter_map(|(idx, ch)| if ch == '\n' { Some(idx + 1) } else { None }));
        Source { text: source, line_starts }
    }

    fn span_start_line(&self, span: Span) -> u32 {
        match self.line_starts.binary_search(&span.start.source_pos()) {
            Ok(idx) => idx as u32 + 1,
            Err(idx) => idx as u32,
        }
    }
}
