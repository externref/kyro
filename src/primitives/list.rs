use crate::interpreter::{runtime_error::RuntimeError, value::Value};
use crate::parser::tokens::Token;
use crate::primitives::PrimitiveMethod;
use std::cell::RefCell;
use std::rc::Rc;

pub fn get_list_method(list: Rc<RefCell<Vec<Value>>>, name: &Token) -> Result<Value, RuntimeError> {
    match name.lexeme.as_str() {
        "len" => Ok(Value::Callable(Rc::new(len(list)))),
        "push" => Ok(Value::Callable(Rc::new(push(list)))),
        "pop" => Ok(Value::Callable(Rc::new(pop(list)))),
        _ => Err(RuntimeError::new(
            name.clone(),
            format!("Undefined list method '{}'.", name.lexeme),
        )),
    }
}

fn len(list: Rc<RefCell<Vec<Value>>>) -> PrimitiveMethod {
    PrimitiveMethod::new("len", 0, move |_, _| {
        Ok(Value::Number(list.borrow().len() as f64))
    })
}

fn push(list: Rc<RefCell<Vec<Value>>>) -> PrimitiveMethod {
    PrimitiveMethod::new("push", 1, move |_, args| {
        list.borrow_mut().push(args[0].clone());
        Ok(Value::Nil)
    })
}

fn pop(list: Rc<RefCell<Vec<Value>>>) -> PrimitiveMethod {
    PrimitiveMethod::new("pop", 0, move |_, _| {
        let val = list.borrow_mut().pop().unwrap_or(Value::Nil);
        Ok(val)
    })
}
