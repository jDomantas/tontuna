use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast;
use super::{Env, Value};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Str {
    pub(crate) chars: Vec<char>,
}

impl Str {
    fn encode(&self) -> String {
        self.chars.iter().copied().collect()
    }

    pub(crate) fn lookup_field(&self, as_value: &Value, field: &str) -> Option<Value> {
        match field {
            "len" => Some(Value::Int(self.chars.len() as i64)),
            "get" => {
                let as_value = as_value.clone();
                Some(Value::NativeFunc(Rc::new(NativeFunc::new1("get", move |idx| {
                    super::intrinsics::string_get(&as_value, idx)
                }))))
            }
            "substring" => {
                let as_value = as_value.clone();
                Some(Value::NativeFunc(Rc::new(NativeFunc::new2("substring", move |idx, len| {
                    super::intrinsics::substring(&as_value, idx, len)
                }))))
            }
            _ => None,
        }
    }
}

impl std::fmt::Debug for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.encode().fmt(f)
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.encode().fmt(f)
    }
}

pub(crate) struct NativeFunc {
    pub(crate) name: String,
    pub(crate) f: Box<dyn Fn(&[Value]) -> Result<Value, String>>,
}

impl NativeFunc {
    pub(crate) fn new(
        name: impl Into<String>,
        f: impl Fn(&[Value]) -> Result<Value, String> + 'static,
    ) -> NativeFunc {
        NativeFunc {
            name: name.into(),
            f: Box::new(f),
        }
    }

    pub(crate) fn new1(
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

    pub(crate) fn new2(
        name: impl Into<String>,
        f: impl Fn(&Value, &Value) -> Result<Value, String> + 'static,
    ) -> NativeFunc {
        let name = name.into();
        NativeFunc {
            name: name.clone(),
            f: Box::new(move |values| {
                match values {
                    [a, b] => f(a, b),
                    _ => Err(format!(
                        "{} expects 2 arguments, got {}",
                        name,
                        values.len(),
                    )),
                }
            }),
        }
    }

    pub(crate) fn new3(
        name: impl Into<String>,
        f: impl Fn(&Value, &Value, &Value) -> Result<Value, String> + 'static,
    ) -> NativeFunc {
        let name = name.into();
        NativeFunc {
            name: name.clone(),
            f: Box::new(move |values| {
                match values {
                    [a, b, c] => f(a, b, c),
                    _ => Err(format!(
                        "{} expects 3 arguments, got {}",
                        name,
                        values.len(),
                    )),
                }
            }),
        }
    }
}

pub(crate) struct UserFunc {
    pub(crate) name: String,
    pub(crate) def: Rc<ast::FnDef>,
    pub(crate) env: Env,
}

impl UserFunc {
    pub(crate) fn new(name: String, def: Rc<ast::FnDef>, env: Env) -> UserFunc {
        UserFunc { name, def, env }
    }
}

pub(crate) struct Struct {
    pub(crate) name: String,
}

pub(crate) struct Instance {
    pub(crate) ty: Rc<Struct>,
    pub(crate) fields: RefCell<HashMap<String, Value>>,
}

impl Instance {
    pub(crate) fn lookup_field(&self, field: &str) -> Option<Value> {
        self.fields.borrow().get(field).cloned()
    }

    pub(crate) fn set_field(&self, field: &str, value: Value) {
        self.fields.borrow_mut().insert(field.to_owned(), value);
    }
}

pub(crate) struct List;
