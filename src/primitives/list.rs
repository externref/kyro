use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, interpreter::Interpreter, runtime_error::RuntimeError, value::Value,
};
use crate::parser::tokens::Token;

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
        self.list.borrow_mut().push(arguments[0].clone());
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
        let idx = match arguments[0] {
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
        let sep = match &arguments[0] {
            Value::String(s) => s.clone(),
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Join separator must be a string.",
                    self.token.clone(),
                ));
            }
        };

        let borrowed = self.list.borrow();
        let mut parts = Vec::new();
        for val in borrowed.iter() {
            parts.push(interpreter.stringify(val));
        }

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
