// MIT License

// Copyright (c) 2026 sarthak

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::{
    interpreter::{
        callable::KyroCallable, interpreter::Interpreter, runtime_error::RuntimeError, value::Value,
    },
    parser::tokens::Token,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
        let keys_list: Vec<Value> = self
            .dict
            .borrow()
            .keys()
            .map(|key| Value::String(key.clone()))
            .collect();
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
        let vals_list: Vec<Value> = self.dict.borrow().values().cloned().collect();
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
        let first_arg = arguments.into_iter().next().unwrap();
        let key_str = match first_arg {
            Value::String(s) => s,
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
