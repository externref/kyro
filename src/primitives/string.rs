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
        let mut args_iter = arguments.into_iter();
        let first_arg = args_iter.next().unwrap();
        let second_arg = args_iter.next().unwrap();

        let start = match first_arg {
            Value::Number(n) => n as usize,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Start index must be a number.",
                    self.token.clone(),
                ));
            }
        };

        let end = match second_arg {
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
        let first_arg = arguments.into_iter().next().unwrap();
        let separator = match &first_arg {
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
        let first_arg = arguments.into_iter().next().unwrap();
        let substring = match &first_arg {
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
