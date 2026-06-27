use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, class::KyroClass, instance::KyroInstance, interpreter::Interpreter,
    runtime_error::RuntimeError, value::Value,
};
use crate::parser::tokens::{Token, TokenType};

pub fn get_module() -> Value {
    let class = Rc::new(KyroClass {
        name: "io".to_string(),
        superclass: None,
        methods: HashMap::new(),
        doc: None,
    });
    let mut fields = HashMap::new();
    fields.insert("__name__".to_string(), Value::String("std:io".to_string()));
    fields.insert("print".to_string(), Value::Callable(Rc::new(Print)));
    fields.insert("println".to_string(), Value::Callable(Rc::new(Println)));
    fields.insert("input".to_string(), Value::Callable(Rc::new(Input)));
    fields.insert("clear".to_string(), Value::Callable(Rc::new(Clear)));
    fields.insert("write_err".to_string(), Value::Callable(Rc::new(WriteErr)));

    let instance = KyroInstance { class, fields };
    Value::Instance(Rc::new(RefCell::new(instance)))
}

pub struct Print;

impl KyroCallable for Print {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let str_val = interpreter.stringify(&arguments[0]);
        print!("{}", str_val);
        let _ = io::stdout().flush();
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "print"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["value".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "value" {
            Some(Ok(Value::String("".to_string())))
        } else {
            None
        }
    }
}

pub struct Println;

impl KyroCallable for Println {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let str_val = interpreter.stringify(&arguments[0]);
        println!("{}", str_val);
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "println"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["value".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "value" {
            Some(Ok(Value::String("".to_string())))
        } else {
            None
        }
    }
}

pub struct Input;

impl KyroCallable for Input {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let str_val = interpreter.stringify(&arguments[0]);
        print!("{}", str_val);

        if let Err(e) = io::stdout().flush() {
            return Err(interpreter.raise_error(
                "TypeError",
                &format!("Failed to flush stdout: {e}"),
                Token::new(TokenType::Identifier, "input".to_string(), None, 0),
            ));
        }

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let trimmed = buffer
                    .trim_end_matches(|c| c == '\r' || c == '\n')
                    .to_string();
                Ok(Value::String(trimmed))
            }
            Err(e) => Err(interpreter.raise_error(
                "ValueError",
                &format!("Failed to read input: {e}"),
                Token::new(TokenType::Identifier, "input".to_string(), None, 0),
            )),
        }
    }

    fn name(&self) -> &str {
        "input"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["prompt".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "prompt" {
            Some(Ok(Value::String("".to_string())))
        } else {
            None
        }
    }
}

pub struct Clear;

impl KyroCallable for Clear {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        print!("\x1b[2J\x1B[1;1H");
        let _ = io::stdout().flush();
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "clear"
    }
}

pub struct WriteErr;

impl KyroCallable for WriteErr {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let str_val = interpreter.stringify(&arguments[0]);
        eprintln!("{}", str_val);
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "write_err"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["value".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "value" {
            Some(Ok(Value::String("".to_string())))
        } else {
            None
        }
    }
}
