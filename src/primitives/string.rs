use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, interpreter::Interpreter, runtime_error::RuntimeError, value::Value,
};
use crate::parser::tokens::Token;

pub fn get_string_method(s: String, name: &Token) -> Result<Value, RuntimeError> {
    match name.lexeme.as_str() {
        "len" => Ok(Value::Callable(Rc::new(LenFn { s }))),
        "slice" => Ok(Value::Callable(Rc::new(SliceFn {
            s,
            token: name.clone(),
        }))),
        "split" => Ok(Value::Callable(Rc::new(SplitFn {
            s,
            token: name.clone(),
        }))),
        "trim" => Ok(Value::Callable(Rc::new(TrimFn { s }))),
        "contains" => Ok(Value::Callable(Rc::new(ContainsFn {
            s,
            token: name.clone(),
        }))),
        "to_lower" => Ok(Value::Callable(Rc::new(ToLowerFn { s }))),
        "to_upper" => Ok(Value::Callable(Rc::new(ToUpperFn { s }))),
        _ => Err(RuntimeError::Error {
            token: name.clone(),
            value: Value::String(format!("Undefined method '{}' on string.", name.lexeme)),
        }),
    }
}

pub struct LenFn {
    pub s: String,
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
        Ok(Value::Number(self.s.chars().count() as f64))
    }

    fn name(&self) -> &str {
        "len"
    }
}

pub struct SliceFn {
    pub s: String,
    pub token: Token,
}

impl KyroCallable for SliceFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let start = match arguments[0] {
            Value::Number(n) => n as usize,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Start index must be a number.",
                    self.token.clone(),
                ));
            }
        };

        let end = match arguments[1] {
            Value::Number(n) => n as usize,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "End index must be a number.",
                    self.token.clone(),
                ));
            }
        };

        let chars: Vec<char> = self.s.chars().collect();
        if start > end || end > chars.len() {
            return Err(interpreter.raise_error(
                "IndexError",
                "String slice indices out of bounds.",
                self.token.clone(),
            ));
        }

        let sliced: String = chars[start..end].iter().collect();
        Ok(Value::String(sliced))
    }

    fn name(&self) -> &str {
        "slice"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["start".to_string(), "end".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "end" {
            Some(Ok(Value::Number(self.s.chars().count() as f64)))
        } else {
            None
        }
    }
}

pub struct SplitFn {
    pub s: String,
    pub token: Token,
}

impl KyroCallable for SplitFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let separator = match &arguments[0] {
            Value::String(sep) => sep,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Separator must be a string.",
                    self.token.clone(),
                ));
            }
        };

        let parts: Vec<Value> = self
            .s
            .split(separator)
            .map(|part| Value::String(part.to_string()))
            .collect();

        Ok(Value::List(Rc::new(RefCell::new(parts))))
    }

    fn name(&self) -> &str {
        "split"
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
            Some(Ok(Value::String(" ".to_string())))
        } else {
            None
        }
    }
}

pub struct TrimFn {
    pub s: String,
}

impl KyroCallable for TrimFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::String(self.s.trim().to_string()))
    }

    fn name(&self) -> &str {
        "trim"
    }
}

pub struct ContainsFn {
    pub s: String,
    pub token: Token,
}

impl KyroCallable for ContainsFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let substring = match &arguments[0] {
            Value::String(sub) => sub,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Substring must be a string.",
                    self.token.clone(),
                ));
            }
        };

        Ok(Value::Bool(self.s.contains(substring)))
    }

    fn name(&self) -> &str {
        "contains"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["substring".to_string()]
    }
}

pub struct ToLowerFn {
    pub s: String,
}

impl KyroCallable for ToLowerFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::String(self.s.to_lowercase()))
    }

    fn name(&self) -> &str {
        "to_lower"
    }
}

pub struct ToUpperFn {
    pub s: String,
}

impl KyroCallable for ToUpperFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::String(self.s.to_uppercase()))
    }

    fn name(&self) -> &str {
        "to_upper"
    }
}
