use crate::interpreter::{runtime_error::RuntimeError, value::Value};
use crate::parser::tokens::Token;
use crate::primitives::PrimitiveMethod;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn get_dict_method(
    dict: Rc<RefCell<HashMap<String, Value>>>,
    name: &Token,
) -> Result<Value, RuntimeError> {
    match name.lexeme.as_str() {
        "len" => Ok(Value::Callable(Rc::new(len(dict)))),
        "remove" => Ok(Value::Callable(Rc::new(remove(dict)))),
        "keys" => Ok(Value::Callable(Rc::new(keys(dict)))),
        _ => Err(RuntimeError::new(
            name.clone(),
            format!("Undefined dictionary method '{}'.", name.lexeme),
        )),
    }
}

fn len(dict: Rc<RefCell<HashMap<String, Value>>>) -> PrimitiveMethod {
    PrimitiveMethod::new("len", 0, move |_, _| {
        Ok(Value::Number(dict.borrow().len() as f64))
    })
}

fn remove(dict: Rc<RefCell<HashMap<String, Value>>>) -> PrimitiveMethod {
    PrimitiveMethod::new("remove", 1, move |_, args| {
        let key_str = match &args[0] {
            Value::String(s) => s.clone(),
            _ => args[0].to_string(),
        };
        let removed = dict.borrow_mut().remove(&key_str).unwrap_or(Value::Nil);
        Ok(removed)
    })
}

fn keys(dict: Rc<RefCell<HashMap<String, Value>>>) -> PrimitiveMethod {
    PrimitiveMethod::new("keys", 0, move |_, _| {
        let borrowed = dict.borrow();
        let mut keys_list = Vec::new();
        for key in borrowed.keys() {
            keys_list.push(Value::String(key.clone()));
        }
        Ok(Value::List(Rc::new(RefCell::new(keys_list))))
    })
}
