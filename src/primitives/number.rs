use crate::interpreter::{runtime_error::RuntimeError, value::Value};
use crate::parser::tokens::Token;
use crate::primitives::PrimitiveMethod;
use std::rc::Rc;

pub fn get_number_method(n: f64, name: &Token) -> Result<Value, RuntimeError> {
    match name.lexeme.as_str() {
        "floor" => Ok(Value::Callable(Rc::new(floor(n)))),
        "ceil" => Ok(Value::Callable(Rc::new(ceil(n)))),
        "round" => Ok(Value::Callable(Rc::new(round(n)))),
        "abs" => Ok(Value::Callable(Rc::new(abs(n)))),
        "to_string" => Ok(Value::Callable(Rc::new(to_string_method(n)))),
        _ => Err(RuntimeError::new(
            name.clone(),
            format!("Undefined number method '{}'.", name.lexeme),
        )),
    }
}

fn floor(n: f64) -> PrimitiveMethod {
    PrimitiveMethod::new("floor", 0, move |_, _| Ok(Value::Number(n.floor())))
}

fn ceil(n: f64) -> PrimitiveMethod {
    PrimitiveMethod::new("ceil", 0, move |_, _| Ok(Value::Number(n.ceil())))
}

fn round(n: f64) -> PrimitiveMethod {
    PrimitiveMethod::new("round", 0, move |_, _| Ok(Value::Number(n.round())))
}

fn abs(n: f64) -> PrimitiveMethod {
    PrimitiveMethod::new("abs", 0, move |_, _| Ok(Value::Number(n.abs())))
}

fn to_string_method(n: f64) -> PrimitiveMethod {
    PrimitiveMethod::new("to_string", 0, move |_, _| Ok(Value::String(n.to_string())))
}
