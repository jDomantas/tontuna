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

pub(crate) struct Evaluator {
    source: String,
    env: HashMap<String, Value>,
}

impl Evaluator {
    pub(crate) fn new(source: String, output: Box<dyn Write>) -> Evaluator {
        let mut env = HashMap::new();
        let output = Rc::new(RefCell::new(output));
        let output2 = output.clone();
        env.insert("print".to_owned(), NativeFunc::new("print", move |values| {
            let mut output = output2.borrow_mut();
            intrinsics::print(values, &mut **output)?;
            Ok(Value::Nil)
        }).into());
        let output2 = output.clone();
        env.insert("println".to_owned(), NativeFunc::new("println", move |values| {
            let mut output = output2.borrow_mut();
            intrinsics::println(values, &mut **output)?;
            Ok(Value::Nil)
        }).into());
        Evaluator { source, env }
    }

    fn eval_statement(&mut self, stmt: &ast::Stmt) -> Result<(), RuntimeError> {
        match stmt {
            ast::Stmt::If { cond, body, tail, .. } => {
                if self.eval_if_cond(cond)? {
                    self.eval_block(&body.contents)?;
                } else {
                    self.eval_if_tail(tail)?;
                }
            }
            ast::Stmt::Expr { expr, .. } => {
                self.eval_expr(expr)?;
            }
            ast::Stmt::For { name, iterable, body, .. } => todo!(),
            ast::Stmt::Return { value, .. } => todo!(),
            ast::Stmt::Let { name, value, .. } => todo!(),
            ast::Stmt::Comment(_) => {}
            ast::Stmt::FnDef(_) => todo!(),
            ast::Stmt::StructDef { name, fns, .. } => todo!(),
            ast::Stmt::Block(block) => {
                self.eval_block(&block.contents)?;
            }
        }
        Ok(())
    }

    fn eval_if_cond(&mut self, cond: &ast::IfCond) -> Result<bool, RuntimeError> {
        match cond {
            ast::IfCond::Expr(e) => self.eval_cond(e),
            ast::IfCond::TypeTest { name, ty, value, .. } => todo!(),
        }
    }

    fn eval_if_tail(&mut self, tail: &ast::IfTail) -> Result<(), RuntimeError> {
        match tail {
            ast::IfTail::None => Ok(()),
            ast::IfTail::Else { body, .. } => self.eval_block(&body.contents),
            ast::IfTail::ElseIf { cond, body, tail, .. } => {
                if self.eval_if_cond(cond)? {
                    self.eval_block(&body.contents)
                } else {
                    self.eval_if_tail(tail)
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: &ast::Expr) -> Result<Value, RuntimeError> {
        match expr {
            ast::Expr::Name { name } => {
                match self.env.get(self.token_source(*name)).cloned() {
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
                let func = self.eval_expr(func)?;
                let mut eval_args = || -> Result<Vec<Value>, RuntimeError> {
                    args.iter()
                        .map(|arg| self.eval_expr(&arg.item))
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
            ast::Expr::Paren { inner, .. } => self.eval_expr(inner),
            ast::Expr::BinOp { lhs, operator, rhs } => {
                match operator.kind {
                    TokenKind::And => {
                        return if self.eval_cond(lhs)? {
                            Ok(self.eval_cond(rhs)?.into())
                        } else {
                            Ok(false.into())
                        };
                    }
                    TokenKind::Or => {
                        return if self.eval_cond(lhs)? {
                            Ok(true.into())
                        } else {
                            Ok(self.eval_cond(rhs)?.into())
                        };
                    }
                    _ => {}
                }
                let lhs = self.eval_expr(lhs)?;
                let rhs = self.eval_expr(rhs)?;
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
                let obj = self.eval_expr(obj)?;
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

    fn eval_cond(&mut self, cond: &ast::Expr) -> Result<bool, RuntimeError> {
        match self.eval_expr(cond)? {
            Value::Bool(b) => Ok(b),
            other => Err(RuntimeError {
                message: format!("condition evaluated to a {}", other.type_name()),
                span: Some(cond.span()),
            }),
        }
    }

    fn eval_block(&mut self, block: &ast::NakedBlock) -> Result<(), RuntimeError> {
        for stmt in &block.stmts {
            self.eval_statement(stmt)?;
        }
        Ok(())
    }

    fn token_source(&self, token: ast::Token) -> &str {
        &self.source[token.span.source_range()]
    }

    pub(crate) fn run_program(&mut self, program: &ast::Program) -> Result<(), RuntimeError> {
        for stmt in &program.stmts {
            self.eval_statement(stmt)?;
        }
        Ok(())
    }
}
