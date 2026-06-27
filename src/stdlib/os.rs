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
    fields.insert("exit".to_string(), Value::Callable(Rc::new(ExitFn)));
    fields.insert("get_pid".to_string(), Value::Callable(Rc::new(GetPidFn)));
    fields.insert("platform".to_string(), Value::Callable(Rc::new(PlatformFn)));
    fields.insert("arch".to_string(), Value::Callable(Rc::new(ArchFn)));
    fields.insert(
        "current_dir".to_string(),
        Value::Callable(Rc::new(CurrentDirFn)),
    );
    fields.insert(
        "set_current_dir".to_string(),
        Value::Callable(Rc::new(SetCurrentDirFn)),
    );
    fields.insert("execute".to_string(), Value::Callable(Rc::new(ExecuteFn)));

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

pub struct ExitFn;

impl KyroCallable for ExitFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let code = match arguments[0] {
            Value::Number(n) => n as i32,
            _ => {
                return Err(RuntimeError::new(
                    dummy_token(),
                    "Exit code must be a number.",
                ));
            }
        };
        std::process::exit(code);
    }

    fn name(&self) -> &str {
        "exit"
    }
}

pub struct GetPidFn;

impl KyroCallable for GetPidFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::Number(std::process::id() as f64))
    }

    fn name(&self) -> &str {
        "get_pid"
    }
}

pub struct PlatformFn;

impl KyroCallable for PlatformFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::String(std::env::consts::OS.to_string()))
    }

    fn name(&self) -> &str {
        "platform"
    }
}

pub struct ArchFn;

impl KyroCallable for ArchFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::String(std::env::consts::ARCH.to_string()))
    }

    fn name(&self) -> &str {
        "arch"
    }
}

pub struct CurrentDirFn;

impl KyroCallable for CurrentDirFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        match std::env::current_dir() {
            Ok(path) => Ok(Value::String(path.to_string_lossy().into_owned())),
            Err(e) => Err(RuntimeError::new(
                dummy_token(),
                format!("Failed to retrieve current working directory: {}", e),
            )),
        }
    }

    fn name(&self) -> &str {
        "current_dir"
    }
}

pub struct SetCurrentDirFn;

impl KyroCallable for SetCurrentDirFn {
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

        match std::env::set_current_dir(path) {
            Ok(_) => Ok(Value::Nil),
            Err(e) => Err(RuntimeError::new(
                dummy_token(),
                format!("Failed to set current working directory: {}", e),
            )),
        }
    }

    fn name(&self) -> &str {
        "set_current_dir"
    }
}

pub struct ExecuteFn;

impl KyroCallable for ExecuteFn {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let command = match &arguments[0] {
            Value::String(s) => s,
            _ => {
                return Err(RuntimeError::new(
                    dummy_token(),
                    "Command must be a string.",
                ));
            }
        };

        let args_list = match &arguments[1] {
            Value::List(list_ref) => list_ref.borrow(),
            _ => {
                return Err(RuntimeError::new(
                    dummy_token(),
                    "Command arguments must be supplied as a list.",
                ));
            }
        };

        let mut cmd_args = Vec::new();
        for val in args_list.iter() {
            match val {
                Value::String(s) => cmd_args.push(s.clone()),
                _ => {
                    return Err(RuntimeError::new(
                        dummy_token(),
                        "All command arguments must be strings.",
                    ));
                }
            }
        }

        match std::process::Command::new(command).args(&cmd_args).output() {
            Ok(output) => {
                let exit_code = output.status.code().unwrap_or(-1) as f64;
                let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

                let mut result = HashMap::new();
                result.insert("exit_code".to_string(), Value::Number(exit_code));
                result.insert("stdout".to_string(), Value::String(stdout));
                result.insert("stderr".to_string(), Value::String(stderr));

                Ok(Value::Dict(Rc::new(RefCell::new(result))))
            }
            Err(e) => Err(RuntimeError::new(
                dummy_token(),
                format!("Failed to execute process '{}': {}", command, e),
            )),
        }
    }

    fn name(&self) -> &str {
        "execute"
    }
}
