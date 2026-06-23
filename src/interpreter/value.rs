use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

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
        }
    }
}
