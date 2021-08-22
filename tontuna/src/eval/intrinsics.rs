use std::io::Write;
use std::rc::Rc;
use std::cmp::Ordering;
use crate::ast;
use super::Value;

fn compare(lhs: &Value, rhs: &Value) -> Option<Ordering> {
    match (lhs, rhs) {
        (Value::Nil, Value::Nil) => Some(Ordering::Equal),
        (Value::Nil, _) | (_, Value::Nil) => None,
        (Value::Int(a), Value::Int(b)) => Some(a.cmp(b)),
        (Value::Int(_), _) | (_, Value::Int(_)) => None,
        (Value::Bool(a), Value::Bool(b)) => Some(a.cmp(b)),
        (Value::Bool(_), _) | (_, Value::Bool(_)) => None,
        (Value::Str(a), Value::Str(b)) => Some(a.cmp(b)),
        (Value::Str(_), _) | (_, Value::Str(_)) => None,
        (Value::NativeFunc(a), Value::NativeFunc(b)) => Rc::ptr_eq(a, b).then(|| Ordering::Equal),
        (Value::NativeFunc(_), _) | (_, Value::NativeFunc(_)) => None,
        (Value::Struct(a), Value::Struct(b)) => Rc::ptr_eq(a, b).then(|| Ordering::Equal),
        (Value::Struct(_), _) | (_, Value::Struct(_)) => None,
        (Value::Instance(a), Value::Instance(b)) => Rc::ptr_eq(a, b).then(|| Ordering::Equal),
        (Value::Instance(_), _) | (_, Value::Instance(_)) => None,
        (Value::List(a), Value::List(b)) => Rc::ptr_eq(a, b).then(|| Ordering::Equal),
        (Value::List(_), _) | (_, Value::List(_)) => None,
        (Value::UserFunc(a), Value::UserFunc(b)) => Rc::ptr_eq(a, b).then(|| Ordering::Equal),
        (Value::UserFunc(_), _) | (_, Value::UserFunc(_)) => None,
        (Value::Stmt(a), Value::Stmt(b)) => Rc::ptr_eq(a, b).then(|| Ordering::Equal),
    }
}

pub(super) fn less(lhs: &Value, rhs: &Value) -> Value {
    compare(lhs, rhs).map(|o| o.is_lt()).unwrap_or(false).into()
}

pub(super) fn less_eq(lhs: &Value, rhs: &Value) -> Value {
    compare(lhs, rhs).map(|o| o.is_le()).unwrap_or(false).into()
}

pub(super) fn greater(lhs: &Value, rhs: &Value) -> Value {
    compare(lhs, rhs).map(|o| o.is_gt()).unwrap_or(false).into()
}

pub(super) fn greater_eq(lhs: &Value, rhs: &Value) -> Value {
    compare(lhs, rhs).map(|o| o.is_ge()).unwrap_or(false).into()
}

pub(super) fn eq(lhs: &Value, rhs: &Value) -> Value {
    (compare(lhs, rhs) == Some(Ordering::Equal)).into()
}

pub(super) fn not_eq(lhs: &Value, rhs: &Value) -> Value {
    (compare(lhs, rhs) != Some(Ordering::Equal)).into()
}

pub(super) fn add(lhs: &Value, rhs: &Value) -> Result<Value, String> {
    match (lhs, rhs) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
        (Value::Str(a), Value::Str(b)) => Ok(format!("{}{}", a, b).as_str().into()),
        (_, _) => Err(format!("can't add {} and {}", lhs.type_name(), rhs.type_name())),
    }
}

pub(super) fn sub(lhs: &Value, rhs: &Value) -> Result<Value, String> {
    match (lhs, rhs) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
        (_, _) => Err(format!("can't subtract {} and {}", lhs.type_name(), rhs.type_name())),
    }
}

pub(super) fn mul(lhs: &Value, rhs: &Value) -> Result<Value, String> {
    match (lhs, rhs) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
        (_, _) => Err(format!("can't multiply {} and {}", lhs.type_name(), rhs.type_name())),
    }
}

pub(super) fn div(lhs: &Value, rhs: &Value) -> Result<Value, String> {
    match (lhs, rhs) {
        (Value::Int(a), Value::Int(0)) => Err("division by zero".into()),
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
        (_, _) => Err(format!("can't divide {} and {}", lhs.type_name(), rhs.type_name())),
    }
}

pub(super) fn print(values: &[Value], output: &mut dyn Write) -> Result<(), String> {
    for value in values {
        do_write(output, value.stringify().as_bytes())?;
    }
    Ok(())
}

pub(super) fn println(values: &[Value], output: &mut dyn Write) -> Result<(), String> {
    print(values, output)?;
    do_write(output, b"\n")
}

fn do_write(output: &mut dyn Write, bytes: &[u8]) -> Result<(), String> {
    output.write_all(bytes).map_err(|_| "io error".to_owned())
}

pub(super) fn substring(s: &Value, start: &Value, len: &Value) -> Result<Value, String> {
    let s = match s {
        Value::Str(s) => s,
        other => return Err(format!(
            "first argument must be Str but was {}",
            other.type_name(),
        )),
    };
    let start = match start {
        Value::Int(s) => *s,
        other => return Err(format!(
            "second argument must be Int but was {}",
            other.type_name(),
        )),
    };
    let len = match len {
        Value::Int(s) => *s,
        other => return Err(format!(
            "third argument must be Int but was {}",
            other.type_name(),
        )),
    };
    if start < 0 || len < 0 || start + len > s.chars.len() as i64 {
        Err("index out of bounds".to_owned())
    } else {
        let s = super::Str {
            chars: s.chars[start as usize..][..len as usize].to_owned(),
        };
        Ok(s.into())
    }
}

pub(super) fn string_get(s: &Value, idx: &Value) -> Result<Value, String> {
    let s = match s {
        Value::Str(s) => s,
        other => return Err(format!(
            "first argument must be Str but was {}",
            other.type_name(),
        )),
    };
    let idx = match idx {
        Value::Int(s) => *s,
        other => return Err(format!(
            "second argument must be Int but was {}",
            other.type_name(),
        )),
    };
    if idx < 0 || idx >= s.chars.len() as i64 {
        Err("index out of bounds".to_owned())
    } else {
        let s = super::Str {
            chars: s.chars[idx as usize..][..1].to_owned(),
        };
        Ok(s.into())
    }
}

pub(super) fn panic(args: &[Value]) -> String {
    if args.len() == 0 {
        return "panic".to_owned();
    }
    let mut message = "panic: ".to_owned();
    for arg in args {
        message.push_str(&arg.stringify());
    }
    message
}

pub(super) fn list_get(s: &Value, idx: &Value) -> Result<Value, String> {
    let s = match s {
        Value::List(s) => s,
        other => return Err(format!(
            "first argument must be List but was {}",
            other.type_name(),
        )),
    };
    let idx = match idx {
        Value::Int(s) => *s,
        other => return Err(format!(
            "second argument must be Int but was {}",
            other.type_name(),
        )),
    };
    if idx < 0 || idx >= s.values.len() as i64 {
        Err("index out of bounds".to_owned())
    } else {
        Ok(s.values[idx as usize].clone())
    }
}

pub(super) fn list_ctor(values: &[Value]) -> Value {
    Value::List(Rc::new(super::List {
        values: values.to_vec(),
    }))
}

pub(super) fn invalid_ctor() -> String {
    "cannot use constructor of builtin type".to_owned()
}

pub(super) fn stmt_children(stmt: &super::Stmt) -> Value {
    let children: Vec<Rc<ast::Stmt>> = match &*stmt.ast {
        ast::Stmt::While { body, .. } => body.contents.stmts.clone(),
        ast::Stmt::If { if_tok, cond, body, tail } => {
            let mut children = body.contents.stmts.clone();
            let mut tail: &ast::IfTail = tail;
            loop {
                match tail {
                    ast::IfTail::None => break,
                    ast::IfTail::Else { body, .. } => {
                        children.extend(body.contents.stmts.iter().cloned());
                        break;
                    }
                    ast::IfTail::ElseIf { body, tail: next_tail, .. } => {
                        children.extend(body.contents.stmts.iter().cloned());
                        tail = &*next_tail;
                    }
                }
            }
            children
        }
        ast::Stmt::Expr { .. } => Vec::new(),
        ast::Stmt::For { body, .. } => body.contents.stmts.clone(),
        ast::Stmt::Return { .. } => Vec::new(),
        ast::Stmt::Let { .. } => Vec::new(),
        ast::Stmt::Comment(c) => {
            let mut children = Vec::new();
            for elem in &c.elements {
                match elem {
                    ast::CommentElem::Text(_) => {},
                    ast::CommentElem::Code { code, .. } => {
                        children.extend(code.stmts.iter().cloned());
                    }
                }
            }
            children
        }
        ast::Stmt::FnDef(f) => f.body.contents.stmts.clone(),
        ast::Stmt::StructDef { .. } => Vec::new(),
        ast::Stmt::Block(b) => b.contents.stmts.clone(),
    };
    Value::List(Rc::new(super::List {
        values: children
            .into_iter()
            .map(|s| Value::Stmt(Rc::new(super::Stmt {
                source: stmt.source.clone(),
                ast: s.clone(),
            })))
            .collect(),
    }))
}
