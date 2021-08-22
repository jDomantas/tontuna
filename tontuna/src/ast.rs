use crate::Span;
pub(crate) use crate::lexer::TokenKind;

#[derive(Debug, Clone)]
pub(crate) struct Program {
    pub(crate) stmts: Vec<Stmt>,
    pub(crate) code_markers: Vec<Token>,
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Name {
        name: Token,
    },
    Number {
        tok: Token,
        value: i64,
    },
    Bool {
        tok: Token,
        value: bool,
    },
    Str {
        tok: Token,
        value: String,
    },
    Nil {
        tok: Token,
    },
    SelfExpr {
        tok: Token,
    },
    Call {
        func: Box<Expr>,
        left_paren: Token,
        args: CommaList<Expr>,
        right_paren: Token,
    },
    Paren {
        left_paren: Token,
        inner: Box<Expr>,
        right_paren: Token,
    },
    BinOp {
        lhs: Box<Expr>,
        operator: Token,
        rhs: Box<Expr>,
    },
    Field {
        obj: Box<Expr>,
        dot: Token,
        field: Token,
    },
}

impl Expr {
    pub(crate) fn span(&self) -> Span {
        match self {
            Expr::Name { name } => name.span,
            Expr::Number { tok, .. } |
            Expr::Bool { tok, .. } |
            Expr::Str { tok, .. } |
            Expr::Nil { tok } |
            Expr::SelfExpr { tok } => tok.span,
            Expr::Call { func, left_paren, args, right_paren } => func.span().merge(right_paren.span),
            Expr::Paren { left_paren, inner, right_paren } => left_paren.span.merge(right_paren.span),
            Expr::BinOp { lhs, operator, rhs } => lhs.span().merge(rhs.span()),
            Expr::Field { obj, dot, field } => obj.span().merge(field.span),
        }
    }
}

pub(crate) type CommaList<T> = Vec<ListItem<T>>;

#[derive(Debug, Clone)]
pub(crate) struct ListItem<T> {
    pub(crate) item: T,
    pub(crate) comma: Option<Token>,
}

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    If {
        if_tok: Token,
        cond: IfCond,
        body: Block,
        tail: IfTail,
    },
    Expr {
        expr: Expr,
        semi: Token,
    },
    For {
        for_tok: Token,
        name: Token,
        in_tok: Token,
        iterable: Expr,
        body: Block,
    },
    Return {
        ret: Token,
        value: Expr,
        semi: Token,
    },
    Let {
        let_tok: Token,
        name: Token,
        eq: Token,
        value: Expr,
        semi: Token,
    },
    Comment(Comment),
    FnDef(FnDef),
    StructDef {
        struct_tok: Token,
        name: Token,
        left_curly: Token,
        fns: Vec<FnDef>,
        right_curly: Token,
    },
    Block(Block),
}

#[derive(Debug, Clone)]
pub(crate) enum IfCond {
    Expr(Expr),
    TypeTest {
        let_tok: Token,
        name: Token,
        colon: Token,
        ty: Expr,
        eq: Token,
        value: Expr,
    },
}

impl IfCond {
    pub(crate) fn span(&self) -> Span {
        match self {
            IfCond::Expr(e) => e.span(),
            IfCond::TypeTest { let_tok, name, colon, ty, eq, value } => let_tok.span.merge(value.span()),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum IfTail {
    None,
    Else {
        else_tok: Token,
        body: Block,
    },
    ElseIf {
        else_tok: Token,
        if_tok: Token,
        cond: IfCond,
        body: Block,
        tail: Box<IfTail>,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct FnDef {
    pub(crate) fn_tok: Token,
    pub(crate) name: Token,
    pub(crate) left_paren: Token,
    pub(crate) params: CommaList<Token>,
    pub(crate) right_paren: Token,
    pub(crate) body: Block,
}

#[derive(Debug, Clone)]
pub(crate) struct Block {
    pub(crate) left_curly: Token,
    pub(crate) contents: NakedBlock,
    pub(crate) right_curly: Token,
}

#[derive(Debug, Clone)]
pub(crate) struct NakedBlock {
    pub(crate) stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub(crate) struct Comment {
    // can be interleaved with or even inside comment elements
    pub(crate) markers: Vec<Token>,
    pub(crate) elements: Vec<CommentElem>,
}

#[derive(Debug, Clone)]
pub(crate) enum CommentElem {
    Text(Token),
    Code {
        // can be inside code contents
        markers: Vec<Token>,
        code: NakedBlock,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Token {
    pub(crate) span: Span,
    pub(crate) kind: TokenKind,
}
