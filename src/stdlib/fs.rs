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

    fields.insert(
        "create_dir".to_string(),
        Value::Callable(Rc::new(CreateDir)),
    );
    fields.insert("read_dir".to_string(), Value::Callable(Rc::new(ReadDir)));

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

    fn parameter_names(&self) -> Vec<String> {
        vec!["path".to_string()]
    }
}

pub struct WriteFile;

impl KyroCallable for WriteFile {
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

    fn parameter_names(&self) -> Vec<String> {
        vec!["path".to_string(), "content".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "content" {
            Some(Ok(Value::String("".to_string())))
        } else {
            None
        }
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

    fn parameter_names(&self) -> Vec<String> {
        vec!["path".to_string()]
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

    fn parameter_names(&self) -> Vec<String> {
        vec!["path".to_string()]
    }
}

pub struct CreateDir;

impl KyroCallable for CreateDir {
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
                    Token::new(TokenType::Identifier, "create_dir".to_string(), None, 0),
                    "First argument to create_dir() must be a string path.",
                ));
            }
        };

        let recursive = match arguments[1] {
            Value::Bool(b) => b,
            _ => {
                return Err(RuntimeError::new(
                    Token::new(TokenType::Identifier, "create_dir".to_string(), None, 0),
                    "Second argument 'recursive' must be a boolean.",
                ));
            }
        };

        let result = if recursive {
            std::fs::create_dir_all(path)
        } else {
            std::fs::create_dir(path)
        };

        match result {
            Ok(_) => Ok(Value::Nil),
            Err(e) => Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "create_dir".to_string(), None, 0),
                format!("Failed to create directory '{path}': {e}"),
            )),
        }
    }

    fn name(&self) -> &str {
        "create_dir"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["path".to_string(), "recursive".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "recursive" {
            Some(Ok(Value::Bool(false)))
        } else {
            None
        }
    }
}

pub struct ReadDir;

impl KyroCallable for ReadDir {
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
                    Token::new(TokenType::Identifier, "read_dir".to_string(), None, 0),
                    "Argument to read_dir() must be a string path.",
                ));
            }
        };

        match std::fs::read_dir(path) {
            Ok(entries) => {
                let mut list = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        let name = entry.file_name().to_string_lossy().into_owned();
                        list.push(Value::String(name));
                    }
                }
                Ok(Value::List(Rc::new(RefCell::new(list))))
            }
            Err(e) => Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "read_dir".to_string(), None, 0),
                format!("Failed to read directory '{path}': {e}"),
            )),
        }
    }

    fn name(&self) -> &str {
        "read_dir"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["path".to_string()]
    }
}
