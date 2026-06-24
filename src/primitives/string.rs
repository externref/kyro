use crate::interpreter::{runtime_error::RuntimeError, value::Value};
use crate::parser::tokens::Token;
use crate::primitives::PrimitiveMethod;
use std::cell::RefCell;
use std::rc::Rc;

pub fn get_string_method(s: String, name: &Token) -> Result<Value, RuntimeError> {
    match name.lexeme.as_str() {
        "len" => Ok(Value::Callable(Rc::new(len(s)))),
        "slice" => Ok(Value::Callable(Rc::new(slice(s, name.clone())))),
        "split" => Ok(Value::Callable(Rc::new(split(s, name.clone())))),
        _ => Err(RuntimeError::new(
            name.clone(),
            format!("Undefined string method '{}'.", name.lexeme),
        )),
    }
}

fn len(s: String) -> PrimitiveMethod {
    PrimitiveMethod::new("len", 0, move |_, _| {
        Ok(Value::Number(s.chars().count() as f64))
    })
}

fn slice(s: String, token: Token) -> PrimitiveMethod {
    PrimitiveMethod::new("slice", 2, move |_, args| {
        let start = match args[0] {
            Value::Number(n) => n as usize,
            _ => {
                return Err(RuntimeError::new(
                    token.clone(),
                    "Start index must be a number.",
                ));
            }
        };
        let end = match args[1] {
            Value::Number(n) => n as usize,
            _ => {
                return Err(RuntimeError::new(
                    token.clone(),
                    "End index must be a number.",
                ));
            }
        };

        let chars: Vec<char> = s.chars().collect();
        if start > end || end > chars.len() {
            return Err(RuntimeError::new(
                token.clone(),
                "String slice indices out of bounds.",
            ));
        }

        let sliced: String = chars[start..end].iter().collect();
        Ok(Value::String(sliced))
    })
}

fn split(s: String, token: Token) -> PrimitiveMethod {
    PrimitiveMethod::new("split", 1, move |_, args| {
        let separator = match &args[0] {
            Value::String(sep) => sep,
            _ => {
                return Err(RuntimeError::new(
                    token.clone(),
                    "Separator must be a string.",
                ));
            }
        };

        let parts: Vec<Value> = s
            .split(separator)
            .map(|part| Value::String(part.to_string()))
            .collect();

        Ok(Value::List(Rc::new(RefCell::new(parts))))
    })
}
