use std::io::Write;
use std::rc::Rc;
use std::cmp::Ordering;
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
        // (Value::NativeFunc(_), _) | (_, Value::NativeFunc(_)) => None,
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