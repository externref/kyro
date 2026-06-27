use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, interpreter::Interpreter, runtime_error::RuntimeError, value::Value,
};
use crate::parser::tokens::Token;

pub fn get_number_method(n: f64, name: &Token) -> Result<Value, RuntimeError> {
    match name.lexeme.as_str() {
        "floor" => Ok(Value::Callable(Rc::new(FloorFn { n }))),
        "ceil" => Ok(Value::Callable(Rc::new(CeilFn { n }))),
        "round" => Ok(Value::Callable(Rc::new(RoundFn { n }))),
        "abs" => Ok(Value::Callable(Rc::new(AbsFn { n }))),
        "to_string" => Ok(Value::Callable(Rc::new(ToStringFn { n }))),
        "clamp" => Ok(Value::Callable(Rc::new(ClampFn {
            n,
            token: name.clone(),
        }))),
        "round_to" => Ok(Value::Callable(Rc::new(RoundToFn {
            n,
            token: name.clone(),
        }))),
        _ => Err(RuntimeError::Error {
            token: name.clone(),
            value: Value::String(format!("Undefined method '{}' on number.", name.lexeme)),
        }),
    }
}

pub struct FloorFn {
    pub n: f64,
}

impl KyroCallable for FloorFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.n.floor()))
    }

    fn name(&self) -> &str {
        "floor"
    }
}

pub struct CeilFn {
    pub n: f64,
}

impl KyroCallable for CeilFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.n.ceil()))
    }

    fn name(&self) -> &str {
        "ceil"
    }
}

pub struct RoundFn {
    pub n: f64,
}

impl KyroCallable for RoundFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.n.round()))
    }

    fn name(&self) -> &str {
        "round"
    }
}

pub struct AbsFn {
    pub n: f64,
}

impl KyroCallable for AbsFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.n.abs()))
    }

    fn name(&self) -> &str {
        "abs"
    }
}

pub struct ToStringFn {
    pub n: f64,
}

impl KyroCallable for ToStringFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::String(self.n.to_string()))
    }

    fn name(&self) -> &str {
        "to_string"
    }
}

pub struct ClampFn {
    pub n: f64,
    pub token: Token,
}

impl KyroCallable for ClampFn {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let min = match arguments[0] {
            Value::Number(val) => val,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Clamp minimum limit must be a number.",
                    self.token.clone(),
                ));
            }
        };

        let max = match arguments[1] {
            Value::Number(val) => val,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Clamp maximum limit must be a number.",
                    self.token.clone(),
                ));
            }
        };

        let clamped = self.n.max(min).min(max);
        Ok(Value::Number(clamped))
    }

    fn name(&self) -> &str {
        "clamp"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["min".to_string(), "max".to_string()]
    }
}

pub struct RoundToFn {
    pub n: f64,
    pub token: Token,
}

impl KyroCallable for RoundToFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let precision = match arguments[0] {
            Value::Number(val) => val as i32,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Round precision limit must be an integer number.",
                    self.token.clone(),
                ));
            }
        };

        if precision < 0 {
            return Err(interpreter.raise_error(
                "ValueError",
                "Round precision limit cannot be negative.",
                self.token.clone(),
            ));
        }

        let factor = 10.0f64.powi(precision);
        let rounded = (self.n * factor).round() / factor;
        Ok(Value::Number(rounded))
    }

    fn name(&self) -> &str {
        "round_to"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["precision".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "precision" {
            Some(Ok(Value::Number(0.0)))
        } else {
            None
        }
    }
}
