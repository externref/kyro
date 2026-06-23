use std::{cell::RefCell,
collections::HashMap,
fmt,
rc::Rc,};

use super::callable::KyroCallable;
use super::class::KyroClass;
use super::instance::KyroInstance;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Callable(Rc<dyn KyroCallable>),
    Class(Rc<KyroClass>),
    Instance(Rc<RefCell<KyroInstance>>),
    List(Rc<RefCell<Vec<Value>>>), 
    Dict(Rc<RefCell<HashMap<String, Value>>>), 
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),

            Value::Callable(callable) => {
                write!(f, "<fn {}>", callable.name())
            }

            Value::Class(class) => {
                write!(f, "<class {}>", class.name)
            }
            Value::Instance(instance) => {
                write!(f, "<instance {}>", instance.borrow().class.name)
            }
            Value::List(list) => {
                let borrowed = list.borrow();
                let elems: Vec<String> = borrowed.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elems.join(", "))
            }
            Value::Dict(dict) => {
                let borrowed = dict.borrow();
                let entries: Vec<String> = borrowed
                    .iter()
                    .map(|(k, v)| format!("\"{k}\": {v}"))
                    .collect();
                write!(f, "{{{}}}", entries.join(", "))
            }
        }
    }
}
