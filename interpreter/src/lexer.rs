use logos::Logos;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Copy, Clone, Logos)]
pub(crate) enum TokenKind {
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("return")]
    Return,
    #[token("struct")]
    Struct,
    #[token("int")]
    Int,
    #[token("bool")]
    Bool,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("str")]
    Str,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("=")]
    Equals,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEq,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEq,
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Name,
    #[regex("[0-9][a-zA-Z0-9_]*")]
    Number,
    #[token("#")]
    CommentMarker,
    CodeMarker,
    #[regex(" +")]
    Space,
    #[regex(r"\r?\n")]
    Newline,
    #[error]
    Error,
    CommentText,
}

pub(crate) fn next_token(source: &str) -> Option<(TokenKind, usize)> {
    let mut lexer = TokenKind::lexer(source);
    let token = lexer.next()?;
    let len = source.len() - lexer.remainder().len();
    Some((token, len))
}
