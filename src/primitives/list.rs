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
use std::{cell::RefCell, rc::Rc};

pub fn get_list_method(list: Rc<RefCell<Vec<Value>>>, name: &Token) -> Result<Value, RuntimeError> {
    match name.lexeme.as_str() {
        "len" => Ok(Value::Callable(Rc::new(LenFn { list }))),
        "push" => Ok(Value::Callable(Rc::new(PushFn { list }))),
        "pop" => Ok(Value::Callable(Rc::new(PopFn { list }))),
        "clear" => Ok(Value::Callable(Rc::new(ClearFn { list }))),
        "remove" => Ok(Value::Callable(Rc::new(RemoveFn {
            list,
            token: name.clone(),
        }))),
        "join" => Ok(Value::Callable(Rc::new(JoinFn {
            list,
            token: name.clone(),
        }))),
        _ => Err(RuntimeError::Error {
            token: name.clone(),
            value: Value::String(format!("Undefined method '{}' on list.", name.lexeme)),
        }),
    }
}

pub struct LenFn {
    pub list: Rc<RefCell<Vec<Value>>>,
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
        Ok(Value::Number(self.list.borrow().len() as f64))
    }

    fn name(&self) -> &str {
        "len"
    }
}

pub struct PushFn {
    pub list: Rc<RefCell<Vec<Value>>>,
}

impl KyroCallable for PushFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let first_arg = arguments.into_iter().next().unwrap();
        self.list.borrow_mut().push(first_arg);
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "push"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["value".to_string()]
    }
}

pub struct PopFn {
    pub list: Rc<RefCell<Vec<Value>>>,
}

impl KyroCallable for PopFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let val = self.list.borrow_mut().pop().unwrap_or(Value::Nil);
        Ok(val)
    }

    fn name(&self) -> &str {
        "pop"
    }
}

pub struct ClearFn {
    pub list: Rc<RefCell<Vec<Value>>>,
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
        self.list.borrow_mut().clear();
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "clear"
    }
}

pub struct RemoveFn {
    pub list: Rc<RefCell<Vec<Value>>>,
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
        let idx = match first_arg {
            Value::Number(n) => n as usize,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "List index must be a number.",
                    self.token.clone(),
                ));
            }
        };

        let mut borrowed = self.list.borrow_mut();
        if idx >= borrowed.len() {
            return Err(interpreter.raise_error(
                "IndexError",
                "List index out of bounds.",
                self.token.clone(),
            ));
        }

        Ok(borrowed.remove(idx))
    }

    fn name(&self) -> &str {
        "remove"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["index".to_string()]
    }
}

pub struct JoinFn {
    pub list: Rc<RefCell<Vec<Value>>>,
    pub token: Token,
}

impl KyroCallable for JoinFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let first_arg = arguments.into_iter().next().unwrap();
        let sep = match first_arg {
            Value::String(s) => s,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Join separator must be a string.",
                    self.token.clone(),
                ));
            }
        };

        let borrowed = self.list.borrow();
        let parts: Vec<String> = borrowed
            .iter()
            .map(|val| interpreter.stringify(val))
            .collect();

        Ok(Value::String(parts.join(&sep)))
    }

    fn name(&self) -> &str {
        "join"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["separator".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "separator" {
            Some(Ok(Value::String("".to_string())))
        } else {
            None
        }
    }
}
