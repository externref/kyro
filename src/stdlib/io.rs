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
    });
    let mut fields = HashMap::new();
    fields.insert("print".to_string(), Value::Callable(Rc::new(Print)));
    fields.insert("println".to_string(), Value::Callable(Rc::new(Println)));
    fields.insert("input".to_string(), Value::Callable(Rc::new(Input)));

    let instance = KyroInstance { class, fields };
    Value::Instance(Rc::new(RefCell::new(instance)))
}

pub struct Print;

impl KyroCallable for Print {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        print!("{}", arguments[0]);
        let _ = io::stdout().flush();
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "print"
    }
}

pub struct Println;

impl KyroCallable for Println {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        println!("{}", arguments[0]);
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "println"
    }
}

pub struct Input;

impl KyroCallable for Input {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let prompt_val = &arguments[0];
        print!("{}", prompt_val);

        if let Err(e) = io::stdout().flush() {
            return Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "input".to_string(), None, 0),
                format!("Failed to flush stdout: {e}"),
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
            Err(e) => Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "input".to_string(), None, 0),
                format!("Failed to read input: {e}"),
            )),
        }
    }

    fn name(&self) -> &str {
        "input"
    }
}
