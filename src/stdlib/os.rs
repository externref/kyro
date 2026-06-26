use crate::parser::tokens::{Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, class::KyroClass, instance::KyroInstance, interpreter::Interpreter,
    runtime_error::RuntimeError, value::Value,
};

pub fn get_module() -> Value {
    let class = Rc::new(KyroClass {
        name: "os".to_string(),
        superclass: None,
        methods: HashMap::new(),
        doc: None,
    });
    let mut fields = HashMap::new();
    fields.insert("__name__".to_string(), Value::String("std:os".to_string()));
    fields.insert("args".to_string(), Value::Callable(Rc::new(ArgsFn)));
    fields.insert(
        "load_dotenv".to_string(),
        Value::Callable(Rc::new(LoadDotenvFn)),
    );
    fields.insert("get_env".to_string(), Value::Callable(Rc::new(GetEnvFn)));
    fields.insert("set_env".to_string(), Value::Callable(Rc::new(SetEnvFn)));
    fields.insert("get_envs".to_string(), Value::Callable(Rc::new(GetEnvsFn)));

    let instance = KyroInstance { class, fields };
    Value::Instance(Rc::new(RefCell::new(instance)))
}

fn dummy_token() -> Token {
    Token {
        r#type: TokenType::Identifier,
        lexeme: "native".to_string(),
        literal: None,
        line: 0,
    }
}

pub struct ArgsFn;

impl KyroCallable for ArgsFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let args: Vec<Value> = std::env::args().map(|s| Value::String(s)).collect();
        Ok(Value::List(Rc::new(RefCell::new(args))))
    }

    fn name(&self) -> &str {
        "args"
    }
}

pub struct LoadDotenvFn;

impl KyroCallable for LoadDotenvFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let path = match &arguments[0] {
            Value::String(s) => s,
            _ => return Err(RuntimeError::new(dummy_token(), "Path must be a string.")),
        };

        let content = std::fs::read_to_string(path).map_err(|e| {
            RuntimeError::new(dummy_token(), format!("Failed to read dotenv file: {}", e))
        })?;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((key, val)) = trimmed.split_once('=') {
                let key = key.trim();
                let val = val.trim().trim_matches(|c| c == '"' || c == '\'');
                unsafe {
                    std::env::set_var(key, val);
                }
            }
        }

        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "load_dotenv"
    }
}

pub struct GetEnvFn;

impl KyroCallable for GetEnvFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let key = match &arguments[0] {
            Value::String(s) => s,
            _ => {
                return Err(RuntimeError::new(
                    dummy_token(),
                    "Environment variable key must be a string.",
                ));
            }
        };

        match std::env::var(key) {
            Ok(val) => Ok(Value::String(val)),
            Err(_) => Ok(Value::Nil),
        }
    }

    fn name(&self) -> &str {
        "get_env"
    }
}

pub struct SetEnvFn;

impl KyroCallable for SetEnvFn {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let key = match &arguments[0] {
            Value::String(s) => s,
            _ => return Err(RuntimeError::new(dummy_token(), "Key must be a string.")),
        };
        let value = match &arguments[1] {
            Value::String(s) => s,
            _ => return Err(RuntimeError::new(dummy_token(), "Value must be a string.")),
        };

        unsafe {
            std::env::set_var(key, value);
        }
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "set_env"
    }
}

pub struct GetEnvsFn;

impl KyroCallable for GetEnvsFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let mut env_list = Vec::new();
        for (key, val) in std::env::vars() {
            let pair = vec![Value::String(key), Value::String(val)];
            env_list.push(Value::List(Rc::new(RefCell::new(pair))));
        }

        Ok(Value::List(Rc::new(RefCell::new(env_list))))
    }

    fn name(&self) -> &str {
        "get_envs"
    }
}
