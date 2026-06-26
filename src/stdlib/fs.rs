use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, class::KyroClass, instance::KyroInstance, interpreter::Interpreter,
    runtime_error::RuntimeError, value::Value,
};
use crate::parser::tokens::{Token, TokenType};

pub fn get_module() -> Value {
    let class = Rc::new(KyroClass {
        name: "fs".to_string(),
        superclass: None,
        methods: HashMap::new(),
        doc: None,
    });
    let mut fields = HashMap::new();

    fields.insert("__name__".to_string(), Value::String("std:fs".to_string()));
    fields.insert("read_file".to_string(), Value::Callable(Rc::new(ReadFile)));
    fields.insert(
        "write_file".to_string(),
        Value::Callable(Rc::new(WriteFile)),
    );
    fields.insert("exists".to_string(), Value::Callable(Rc::new(Exists)));
    fields.insert(
        "remove_file".to_string(),
        Value::Callable(Rc::new(RemoveFile)),
    );

    let instance = KyroInstance { class, fields };
    Value::Instance(Rc::new(RefCell::new(instance)))
}

pub struct ReadFile;

impl KyroCallable for ReadFile {
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
            _ => {
                return Err(RuntimeError::new(
                    Token::new(TokenType::Identifier, "read_file".to_string(), None, 0),
                    "Argument to read_file() must be a string path.",
                ));
            }
        };

        match std::fs::read_to_string(path) {
            Ok(content) => Ok(Value::String(content)),
            Err(e) => Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "read_file".to_string(), None, 0),
                format!("Failed to read file '{path}': {e}"),
            )),
        }
    }

    fn name(&self) -> &str {
        "read_file"
    }
}

pub struct WriteFile;

impl KyroCallable for WriteFile {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let path = match &arguments[0] {
            Value::String(s) => s,
            _ => {
                return Err(RuntimeError::new(
                    Token::new(TokenType::Identifier, "write_file".to_string(), None, 0),
                    "First argument to write_file() must be a string path.",
                ));
            }
        };

        let content = &arguments[1].to_string();

        match std::fs::write(path, content) {
            Ok(_) => Ok(Value::Nil),
            Err(e) => Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "write_file".to_string(), None, 0),
                format!("Failed to write to file '{path}': {e}"),
            )),
        }
    }

    fn name(&self) -> &str {
        "write_file"
    }
}

pub struct Exists;

impl KyroCallable for Exists {
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
            _ => {
                return Err(RuntimeError::new(
                    Token::new(TokenType::Identifier, "exists".to_string(), None, 0),
                    "Argument to exists() must be a string path.",
                ));
            }
        };

        let path_exists = Path::new(path).exists();
        Ok(Value::Bool(path_exists))
    }

    fn name(&self) -> &str {
        "exists"
    }
}

pub struct RemoveFile;

impl KyroCallable for RemoveFile {
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
            _ => {
                return Err(RuntimeError::new(
                    Token::new(TokenType::Identifier, "remove_file".to_string(), None, 0),
                    "Argument to remove_file() must be a string path.",
                ));
            }
        };

        match std::fs::remove_file(path) {
            Ok(_) => Ok(Value::Nil),
            Err(e) => Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "remove_file".to_string(), None, 0),
                format!("Failed to delete file '{path}': {e}"),
            )),
        }
    }

    fn name(&self) -> &str {
        "remove_file"
    }
}
