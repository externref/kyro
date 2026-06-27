use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, interpreter::Interpreter, runtime_error::RuntimeError, value::Value,
};
use crate::parser::tokens::Token;

pub fn get_dict_method(
    dict: Rc<RefCell<HashMap<String, Value>>>,
    name: &Token,
) -> Result<Value, RuntimeError> {
    match name.lexeme.as_str() {
        "len" => Ok(Value::Callable(Rc::new(LenFn { dict }))),
        "keys" => Ok(Value::Callable(Rc::new(KeysFn { dict }))),
        "values" => Ok(Value::Callable(Rc::new(ValuesFn { dict }))),
        "clear" => Ok(Value::Callable(Rc::new(ClearFn { dict }))),
        "remove" => Ok(Value::Callable(Rc::new(RemoveFn {
            dict,
            token: name.clone(),
        }))),
        _ => Err(RuntimeError::Error {
            token: name.clone(),
            value: Value::String(format!("Undefined method '{}' on dictionary.", name.lexeme)),
        }),
    }
}

pub struct LenFn {
    pub dict: Rc<RefCell<HashMap<String, Value>>>,
}

impl KyroCallable for LenFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.dict.borrow().len() as f64))
    }

    fn name(&self) -> &str {
        "len"
    }
}

pub struct KeysFn {
    pub dict: Rc<RefCell<HashMap<String, Value>>>,
}

impl KyroCallable for KeysFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let borrowed = self.dict.borrow();
        let mut keys_list = Vec::new();
        for key in borrowed.keys() {
            keys_list.push(Value::String(key.clone()));
        }
        Ok(Value::List(Rc::new(RefCell::new(keys_list))))
    }

    fn name(&self) -> &str {
        "keys"
    }
}

pub struct ValuesFn {
    pub dict: Rc<RefCell<HashMap<String, Value>>>,
}

impl KyroCallable for ValuesFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let borrowed = self.dict.borrow();
        let mut vals_list = Vec::new();
        for val in borrowed.values() {
            vals_list.push(val.clone());
        }
        Ok(Value::List(Rc::new(RefCell::new(vals_list))))
    }

    fn name(&self) -> &str {
        "values"
    }
}

pub struct ClearFn {
    pub dict: Rc<RefCell<HashMap<String, Value>>>,
}

impl KyroCallable for ClearFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        self.dict.borrow_mut().clear();
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "clear"
    }
}

pub struct RemoveFn {
    pub dict: Rc<RefCell<HashMap<String, Value>>>,
    pub token: Token,
}

impl KyroCallable for RemoveFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let key_str = match &arguments[0] {
            Value::String(s) => s.clone(),
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Dictionary key must be a string.",
                    self.token.clone(),
                ));
            }
        };
        let removed = self
            .dict
            .borrow_mut()
            .remove(&key_str)
            .unwrap_or(Value::Nil);
        Ok(removed)
    }

    fn name(&self) -> &str {
        "remove"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["key".to_string()]
    }
}
