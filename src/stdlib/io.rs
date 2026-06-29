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
        callable::KyroCallable, class::KyroClass, instance::KyroInstance, interpreter::Interpreter,
        runtime_error::RuntimeError, value::Value,
    },
    parser::tokens::{Token, TokenType},
};
use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Write},
    rc::Rc,
};

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
        let first_arg = arguments.into_iter().next().unwrap();
        let str_val = interpreter.stringify(&first_arg);
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
        let first_arg = arguments.into_iter().next().unwrap();
        let str_val = interpreter.stringify(&first_arg);
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
        let first_arg = arguments.into_iter().next().unwrap();
        let str_val = interpreter.stringify(&first_arg);
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
        let first_arg = arguments.into_iter().next().unwrap();
        let str_val = interpreter.stringify(&first_arg);
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
