mod intrinsics;

use std::{cell::RefCell, collections::HashMap, io::Write, rc::Rc};
use crate::{ast::{self, TokenKind}, Span};

#[derive(Clone)]
pub(crate) enum Value {
    Nil,
    Int(i64),
    Bool(bool),
    Str(Rc<str>),
    NativeFunc(Rc<NativeFunc>),
}

impl From<NativeFunc> for Value {
    fn from(v: NativeFunc) -> Self {
        Self::NativeFunc(Rc::new(v))
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Value {
        Self::Str(v.into())
    }
}

impl Value {
    fn type_name(&self) -> String {
        match self {
            Value::Nil => "nil".to_owned(),
            Value::Int(_) => "Int".to_owned(),
            Value::Bool(_) => "Bool".to_owned(),
            Value::Str(_) => "Str".to_owned(),
            Value::NativeFunc(_) => "NativeFunc".to_owned(),
        }
    }

    fn lookup_field(&self, _field: &str) -> Option<Value> {
        None
    }

    fn set_field(&self, field: &str, value: Value) -> Result<(), String> {
        Err(format!("{} cannot have fields", self.type_name()))
    }

    fn is_callable(&self) -> bool {
        matches!(self, Value::NativeFunc(_))
    }

    fn call(&self, args: &[Value]) -> Result<Value, String> {
        match self {
            Value::NativeFunc(f) => (f.f)(args),
            _ => panic!("tried to call non-callable"),
        }
    }

    fn stringify(&self) -> String {
        match self {
            Value::Nil => "nil".to_owned(),
            Value::Int(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            Value::Str(x) => <str as ToOwned>::to_owned(x),
            Value::NativeFunc(f) => format!("native<{}>", f.name),
        }
    }
}

pub(crate) struct NativeFunc {
    name: String,
    f: Box<dyn Fn(&[Value]) -> Result<Value, String>>,
}

impl NativeFunc {
    fn new(
        name: impl Into<String>,
        f: impl Fn(&[Value]) -> Result<Value, String> + 'static,
    ) -> NativeFunc {
        NativeFunc {
            name: name.into(),
            f: Box::new(f),
        }
    }

    fn new1(
        name: impl Into<String>,
        f: impl Fn(&Value) -> Result<Value, String> + 'static,
    ) -> NativeFunc {
        let name = name.into();
        NativeFunc {
            name: name.clone(),
            f: Box::new(move |values| {
                match values {
                    [single] => f(single),
                    _ => Err(format!(
                        "{} expects 1 argument, got {}",
                        name,
                        values.len(),
                    )),
                }
            }),
        }
    }
}

pub(crate) struct RuntimeError {
    pub(crate) message: String,
    pub(crate) span: Option<Span>,
}

enum EvalStop {
    Error(RuntimeError),
    Return(Value),
}

impl From<RuntimeError> for EvalStop {
    fn from(v: RuntimeError) -> Self {
        Self::Error(v)
    }
}

struct EnvEntry {
    name: String,
    value: RefCell<Value>,
    next: Env,
}

#[derive(Clone)]
enum Env {
    Chain(Rc<EnvEntry>),
    GlobalFence(Rc<Env>),
    Global(Rc<RefCell<HashMap<String, Value>>>),
}

impl Env {
    fn global(globals: HashMap<String, Value>) -> Env {
        Env::Global(Rc::new(globals.into()))
    }

    fn lookup(&self, name: &str) -> Option<Value> {
        let mut this = self;
        loop {
            match this {
                Env::Chain(entry) => {
                    if entry.name == name {
                        return Some(entry.value.borrow().clone());
                    }
                    this = &entry.next;
                }
                Env::GlobalFence(env) => this = env,
                Env::Global(globals) => return globals.borrow().get(name).cloned(),
            }
        }
    }

    fn with_fence(&self) -> Env {
        match self {
            Env::Chain(_) |
            Env::GlobalFence(_) => self.clone(),
            Env::Global(_) => Env::GlobalFence(Rc::new(self.clone())),
        }
    }

    fn define(&self, name: &str, value: Value) -> Env {
        match self {
            Env::Chain { .. } |
            Env::GlobalFence(_) => {
                Env::Chain(Rc::new(EnvEntry {
                    name: name.to_owned(),
                    value: value.into(),
                    next: self.clone(),
                }))
            }
            Env::Global(globals) => {
                globals.borrow_mut().insert(name.to_owned(), value);
                self.clone()
            }
        }
    }

    fn set(&self, name: &str, value: Value) -> Result<(), ()> {
        let mut this = self;
        loop {
            match this {
                Env::Chain(entry) => {
                    if entry.name == name {
                        (*entry.value.borrow_mut()) = value;
                        return Ok(())
                    }
                    this = &entry.next;
                }
                Env::GlobalFence(env) => this = env,
                Env::Global(globals) => {
                    return match globals.borrow_mut().get_mut(name) {
                        Some(v) => {
                            *v = value;
                            Ok(())
                        }
                        None => {
                            Err(())
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct Evaluator {
    source: String,
    globals: Env,
}

impl Evaluator {
    pub(crate) fn new(source: String, output: Box<dyn Write>) -> Evaluator {
        let mut globals = HashMap::new();
        let output = Rc::new(RefCell::new(output));
        let output2 = output.clone();
        globals.insert("print".to_owned(), NativeFunc::new("print", move |values| {
            let mut output = output2.borrow_mut();
            intrinsics::print(values, &mut **output)?;
            Ok(Value::Nil)
        }).into());
        let output2 = output.clone();
        globals.insert("println".to_owned(), NativeFunc::new("println", move |values| {
            let mut output = output2.borrow_mut();
            intrinsics::println(values, &mut **output)?;
            Ok(Value::Nil)
        }).into());
        Evaluator { source, globals: Env::global(globals) }
    }

    fn eval_statement(&mut self, stmt: &ast::Stmt, env: &Env) -> Result<Env, EvalStop> {
        match stmt {
            ast::Stmt::If { cond, body, tail, .. } => {
                if self.eval_if_cond(cond, env)? {
                    self.eval_block(&body.contents, env)?;
                } else {
                    self.eval_if_tail(tail, env)?;
                }
            }
            ast::Stmt::Expr { expr, .. } => {
                self.eval_expr(expr, env)?;
            }
            ast::Stmt::For { name, iterable, body, .. } => todo!(),
            ast::Stmt::Return { value, .. } => todo!(),
            ast::Stmt::Let { name, value, .. } => {
                let value = self.eval_expr(value, env)?;
                return Ok(env.define(self.token_source(*name), value));
            }
            ast::Stmt::Comment(_) => {}
            ast::Stmt::FnDef(_) => todo!(),
            ast::Stmt::StructDef { name, fns, .. } => todo!(),
            ast::Stmt::Block(block) => {
                self.eval_block(&block.contents, env)?;
            }
        }
        Ok(env.clone())
    }

    fn eval_if_cond(&mut self, cond: &ast::IfCond, env: &Env) -> Result<bool, RuntimeError> {
        match cond {
            ast::IfCond::Expr(e) => self.eval_cond(e, env),
            ast::IfCond::TypeTest { name, ty, value, .. } => todo!(),
        }
    }

    fn eval_if_tail(&mut self, tail: &ast::IfTail, env: &Env) -> Result<(), EvalStop> {
        match tail {
            ast::IfTail::None => Ok(()),
            ast::IfTail::Else { body, .. } => self.eval_block(&body.contents, env),
            ast::IfTail::ElseIf { cond, body, tail, .. } => {
                if self.eval_if_cond(cond, env)? {
                    self.eval_block(&body.contents, env)
                } else {
                    self.eval_if_tail(tail, env)
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: &ast::Expr, env: &Env) -> Result<Value, RuntimeError> {
        match expr {
            ast::Expr::Name { name } => {
                match env.lookup(self.token_source(*name)) {
                    Some(value) => Ok(value),
                    None => Err(RuntimeError {
                        message: "undefined variable".to_owned(),
                        span: Some(name.span),
                    }),
                }
            }
            ast::Expr::Number { value, .. } => Ok((*value).into()),
            ast::Expr::Bool { value, .. } => Ok((*value).into()),
            ast::Expr::Str { value, .. } => Ok(value.as_str().into()),
            ast::Expr::Nil { .. } => Ok(Value::Nil),
            ast::Expr::SelfExpr { tok } => todo!(),
            ast::Expr::Call { func, args, .. } => {
                let func = self.eval_expr(func, env)?;
                let mut eval_args = || -> Result<Vec<Value>, RuntimeError> {
                    args.iter()
                        .map(|arg| self.eval_expr(&arg.item, env))
                        .collect()
                };
                match func {
                    Value::NativeFunc(f) => {
                        let args = eval_args()?;
                        (f.f)(&args).map_err(|message| RuntimeError {
                            message,
                            span: Some(expr.span()),
                        })
                    }
                    other => {
                        Err(RuntimeError {
                            message: format!("{} cannot be called", other.type_name()),
                            span: Some(expr.span()),
                        })
                    }
                }
            }
            ast::Expr::Paren { inner, .. } => self.eval_expr(inner, env),
            ast::Expr::BinOp { lhs, operator, rhs } => {
                match operator.kind {
                    TokenKind::And => {
                        return if self.eval_cond(lhs, env)? {
                            Ok(self.eval_cond(rhs, env)?.into())
                        } else {
                            Ok(false.into())
                        };
                    }
                    TokenKind::Or => {
                        return if self.eval_cond(lhs, env)? {
                            Ok(true.into())
                        } else {
                            Ok(self.eval_cond(rhs, env)?.into())
                        };
                    }
                    _ => {}
                }
                let lhs = self.eval_expr(lhs, env)?;
                let rhs = self.eval_expr(rhs, env)?;
                let result = match operator.kind {
                    TokenKind::Plus => intrinsics::add(&lhs, &rhs),
                    TokenKind::Minus => intrinsics::sub(&lhs, &rhs),
                    TokenKind::Star => intrinsics::mul(&lhs, &rhs),
                    TokenKind::Slash => intrinsics::div(&lhs, &rhs),
                    TokenKind::Less => Ok(intrinsics::less(&lhs, &rhs)),
                    TokenKind::LessEq => Ok(intrinsics::less_eq(&lhs, &rhs)),
                    TokenKind::Greater => Ok(intrinsics::greater(&lhs, &rhs)),
                    TokenKind::GreaterEq => Ok(intrinsics::greater_eq(&lhs, &rhs)),
                    TokenKind::EqEq => Ok(intrinsics::eq(&lhs, &rhs)),
                    TokenKind::NotEq => Ok(intrinsics::not_eq(&lhs, &rhs)),
                    x => panic!("invalid operator: {:?}", x),
                };
                result.map_err(|message| RuntimeError {
                    message,
                    span: Some(expr.span()),
                })
            }
            ast::Expr::Field { obj, field, .. } => {
                let obj = self.eval_expr(obj, env)?;
                let field_name = self.token_source(*field);
                match obj.lookup_field(field_name) {
                    Some(value) => Ok(value),
                    None => Err(RuntimeError {
                        message: format!(
                            "{} does not have field `{}`",
                            obj.type_name(),
                            field_name,
                        ),
                        span: Some(field.span),
                    }),
                }
            }
        }
    }

    fn eval_cond(&mut self, cond: &ast::Expr, env: &Env) -> Result<bool, RuntimeError> {
        match self.eval_expr(cond, env)? {
            Value::Bool(b) => Ok(b),
            other => Err(RuntimeError {
                message: format!("condition evaluated to a {}", other.type_name()),
                span: Some(cond.span()),
            }),
        }
    }

    fn eval_block(&mut self, block: &ast::NakedBlock, env: &Env) -> Result<(), EvalStop> {
        let mut env = env.with_fence();
        for stmt in &block.stmts {
            env = self.eval_statement(stmt, &env)?;
        }
        Ok(())
    }

    fn token_source(&self, token: ast::Token) -> &str {
        &self.source[token.span.source_range()]
    }

    pub(crate) fn run_program(&mut self, program: &ast::Program) -> Result<(), RuntimeError> {
        let mut env = self.globals.clone();
        for stmt in &program.stmts {
            match self.eval_statement(stmt, &env) {
                Ok(e) => env = e,
                Err(EvalStop::Error(e)) => return Err(e),
                Err(EvalStop::Return(_)) => panic!("return outside of function"),
            }
        }
        Ok(())
    }
}
