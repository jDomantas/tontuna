use crate::{
    ast::{self, TokenKind},
    Pos,
    Span,
};

#[derive(Debug)]
pub(crate) struct Error {
    pub(crate) span: Span,
    pub(crate) message: String,
}

type Result<T> = std::result::Result<T, Error>;

struct Line<'a> {
    start_pos: Pos,
    text: &'a str,
    levels: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Marker {
    Code,
    Comment,
}

impl Marker {
    fn other(self) -> Marker {
        match self {
            Marker::Code => Marker::Comment,
            Marker::Comment => Marker::Code,
        }
    }

    fn text(self) -> &'static str {
        match self {
            Marker::Code => ">",
            Marker::Comment => "#",
        }
    }

    fn into_token_kind(self) -> TokenKind {
        match self {
            Marker::Code => TokenKind::CodeMarker,
            Marker::Comment => TokenKind::CommentMarker,
        }
    }
}

fn find_leading_marker(line: &str) -> Option<(Marker, usize)> {
    let suffix = line.trim_start_matches(|c: char| c.is_ascii_whitespace());
    let kind = match suffix.chars().next() {
        Some('#') => Marker::Comment,
        Some('>') => Marker::Code,
        _ => return None,
    };
    let offset = line.len() - suffix.len();
    Some((kind, offset))
}

#[test]
fn find_leading_marker_test() {
    fn check(text: &str, expected: Option<(Marker, usize)>) {
        assert_eq!(expected, find_leading_marker(text));
    }
    check("#  ", Some((Marker::Comment, 0)));
    check("##  ", Some((Marker::Comment, 0)));
    check("  #  ", Some((Marker::Comment, 2)));
    check(" > ", Some((Marker::Code, 1)));
    check("", None);
    check("< # >", None);
    check("foo # ", None);
}

impl<'a> Line<'a> {
    fn new(idx: u32, text: &'a str, first_level: Marker) -> Line<'a> {
        let mut levels = 0;
        let mut trail_text = text;
        let mut level = first_level;
        while let Some((marker, offset)) = find_leading_marker(trail_text) {
            if marker == level {
                trail_text = &trail_text[(offset + 1)..];
                level = level.other();
                levels += 1;
            } else {
                break;
            }
        }
        Line { start_pos: Pos::new(idx + 1, 1), text, levels }
    }

    fn leading_marker(&self, level: u32) -> ast::Token {
        assert!(self.levels != 0);
        let (kind, offset) = find_leading_marker(self.text).unwrap();
        let span = Span::new(
            self.start_pos.plus_text(&self.text[..offset]),
            self.start_pos.plus_text(&self.text[..(offset + 1)]),
        );
        ast::Token { span, kind: kind.into_token_kind() }
    }

    fn strip_one_marker(&self) -> (ast::Token, Line<'a>) {
        assert!(self.levels != 0);
        let (kind, offset) = find_leading_marker(self.text).unwrap();
        let token_start = self.start_pos.plus_text(&self.text[..offset]);
        let token_end = token_start.plus_text(kind.text());
        let token = ast::Token {
            kind: kind.into_token_kind(),
            span: Span::new(token_start, token_end),
        };
        let line = Line {
            start_pos: token_end,
            text: &self.text[(offset + 1)..],
            levels: self.levels - 1,
        };
        (token, line)
    }
}

pub(crate) fn parse(source: &str) -> Result<ast::Program> {
    let first_marker = detect_first_marker_type(source)?;
    let lines = source
        .lines()
        .enumerate()
        .map(|(idx, line)| Line::new(idx as u32, line, first_marker))
        .collect::<Vec<_>>();
    match first_marker {
        Marker::Comment => {
            let code = parse_code(&lines)?;
            Ok(ast::Program {
                stmts: code.stmts,
            })
        }
        Marker::Code => {
            let comment = parse_comment(&lines, Vec::new())?.elements;
            todo!()
        }
    }
}

fn detect_first_marker_type(source: &str) -> Result<Marker> {
    let mut first_marker = None;
    for (idx, line) in source.lines().enumerate() {
        if let Some((marker, offset)) = find_leading_marker(line) {
            let pos = Pos::new(idx as u32 + 1, offset as u32 + 1);
            match first_marker {
                Some((first, _first_pos)) if first != marker => {
                    return Err(Error {
                        message: "inconsistent file mode".to_owned(),
                        span: Span::new(pos, pos.plus_text(marker.text())),
                    });
                }
                Some(_) => continue,
                None => first_marker = Some((
                    marker,
                    pos,
                )),
            }
        }
    }
    Ok(first_marker.map(|(kind, _)| kind).unwrap_or(Marker::Comment))
}

fn parse_comment(mut lines: &[Line<'_>], markers: Vec<ast::Token>) -> Result<ast::Comment> {
    let mut elements = Vec::new();
    while lines.len() > 0 {
        let first_line = &lines[0];
        if first_line.levels == 0 {
            let span = Span::new(
                first_line.start_pos,
                first_line.start_pos.plus_text(first_line.text),
            );
            elements.push(ast::CommentElem::Text(ast::Token {
                span,
                kind: TokenKind::CommentText,
            }));
            lines = &lines[1..];
        } else {
            let mut code_lines = Vec::new();
            let mut code_markers = Vec::new();
            while lines.get(0).map(|l| l.levels > 0).unwrap_or(false) {
                let (marker, line) = lines[0].strip_one_marker();
                code_lines.push(line);
                code_markers.push(marker);
                lines = &lines[1..];
            }
            let code = parse_code(&code_lines)?;
            elements.push(ast::CommentElem::Code {
                markers: code_markers,
                code,
            });
        }
    }
    Ok(ast::Comment {
        markers,
        elements,
    })
}

fn parse_code(lines: &[Line<'_>]) -> Result<ast::NakedBlock> {
    todo!()
}

struct Parser {

}

impl Parser {
    fn peek(&mut self) -> Option<ast::TokenKind> {
        todo!()
    }

    fn consume(&mut self) -> Option<ast::Token> {
        todo!()
    }

    fn check(&mut self, kind: TokenKind) -> Option<ast::Token> {
        if self.peek() == Some(kind) {
            self.consume()
        } else {
            None
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<ast::Token> {
        if let Some(tok) = self.check(kind) {
            Ok(tok)
        } else {
            todo!("parse error");
        }
    }

    fn parse_expr(&mut self, min_prec: Prec) -> Result<ast::Expr> {
        let mut expr = self.parse_atom_expr()?;
        while let Some(kind) = self.peek() {
            match binop_prec(kind) {
                Some((prec, rhs_prec)) if prec >= min_prec => {
                    let operator = self.expect(kind).unwrap();
                    let rhs = self.parse_expr(rhs_prec)?;
                    expr = ast::Expr::BinOp {
                        lhs: Box::new(expr),
                        operator,
                        rhs: Box::new(rhs),
                    };
                }
                Some(_) => break,
                None if kind == TokenKind::Dot && Prec::CallField >= min_prec => {
                    let dot = self.expect(TokenKind::Dot).unwrap();
                    let field = self.expect(TokenKind::Name)?;
                    expr = ast::Expr::Field {
                        obj: Box::new(expr),
                        dot,
                        field,
                    };
                }
                None if kind == TokenKind::LeftParen && Prec::CallField >= min_prec => {
                    let left_paren = self.expect(TokenKind::LeftParen).unwrap();
                    let args = self.parse_list(|p| p.parse_expr(Prec::Min))?;
                    let right_paren = self.expect(TokenKind::RightParen)?;
                    expr = ast::Expr::Call {
                        func: Box::new(expr),
                        left_paren,
                        args,
                        right_paren,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_list<T>(&mut self, parse_item: impl Fn(&mut Self) -> Result<T>) -> Result<ast::CommaList<T>> {
        let mut list = Vec::new();
        loop {
            if self.peek() == Some(TokenKind::RightParen) {
                return Ok(list);
            }
            let item = parse_item(self)?;
            if let Some(comma) = self.check(TokenKind::Comma) {
                list.push(ast::ListItem { item, comma: Some(comma) });
            } else {
                list.push(ast::ListItem { item, comma: None });
                return Ok(list);
            }
        }
    }

    fn parse_atom_expr(&mut self) -> Result<ast::Expr> {
        if let Some(name) = self.check(TokenKind::Name) {
            Ok(ast::Expr::Name { name })
        } else if let Some(tok) = self.check(TokenKind::Number) {
            match self.token_source(tok).parse::<i64>() {
                Ok(value) => Ok(ast::Expr::Number { tok, value }),
                Err(_) => return Err(Error {
                    span: tok.span,
                    message: "invalid number".to_owned(),
                }),
            }
        } else if let Some(tok) = self.check(TokenKind::False) {
            Ok(ast::Expr::Bool { tok, value: false })
        } else if let Some(tok) = self.check(TokenKind::True) {
            Ok(ast::Expr::Bool { tok, value: true })
        } else if let Some(left_paren) = self.check(TokenKind::LeftParen) {
            let inner = self.parse_expr(Prec::Min)?;
            let right_paren = self.expect(TokenKind::RightParen)?;
            Ok(ast::Expr::Paren {
                left_paren,
                inner: Box::new(inner),
                right_paren,
            })
        } else {
            todo!("parse error")
        }
    }

    fn token_source(&self, token: ast::Token) -> &str {
        todo!()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Prec {
    Min,
    Assign,
    Or,
    And,
    Equals,
    Compare,
    AddSub,
    MulDiv,
    CallField,
    Atom,
}

fn binop_prec(token: TokenKind) -> Option<(Prec, Prec)> {
    match token {
        TokenKind::Equals => Some((Prec::Assign, Prec::Assign)),
        TokenKind::Or => Some((Prec::Or, Prec::And)),
        TokenKind::And => Some((Prec::And, Prec::Equals)),
        TokenKind::EqEq |
        TokenKind::NotEq => Some((Prec::Equals, Prec::Compare)),
        TokenKind::Less |
        TokenKind::LessEq |
        TokenKind::Greater |
        TokenKind::GreaterEq => Some((Prec::Compare, Prec::AddSub)),
        TokenKind::Plus |
        TokenKind::Minus => Some((Prec::AddSub, Prec::MulDiv)),
        TokenKind::Star |
        TokenKind::Slash => Some((Prec::MulDiv, Prec::Atom)),
        _ => None,
    }
}
