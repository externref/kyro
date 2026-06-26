use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, class::KyroClass, instance::KyroInstance, interpreter::Interpreter,
    runtime_error::RuntimeError, value::Value,
};
use crate::parser::tokens::{Token, TokenType};

static VERSION: &'static str = include_str!("../.version");

pub fn get_module() -> Value {
    let class = Rc::new(KyroClass {
        name: "util".to_string(),
        superclass: None,
        methods: HashMap::new(),
        doc: Some("Utility functions for the language.".to_string()),
    });
    let mut fields = HashMap::new();
    fields.insert(
        "__name__".to_string(),
        Value::String("std:util".to_string()),
    );
    fields.insert(
        "to_string".to_string(),
        Value::Callable(Rc::new(ToStringFn)),
    );
    fields.insert("to_number".to_string(), Value::Callable(Rc::new(ToNumber)));
    fields.insert("info".to_string(), Value::Callable(Rc::new(InfoFn)));

    let instance = KyroInstance { class, fields };
    Value::Instance(Rc::new(RefCell::new(instance)))
}
pub struct ToStringFn;

impl KyroCallable for ToStringFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::String(arguments[0].to_string()))
    }

    fn name(&self) -> &str {
        "to_string"
    }
}

pub struct ToNumber;

impl KyroCallable for ToNumber {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let token = Token::new(TokenType::Identifier, "to_number".to_string(), None, 0);
        match &arguments[0] {
            Value::Number(n) => Ok(Value::Number(*n)),
            Value::String(s) => match s.trim().parse::<f64>() {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Err(RuntimeError::new(
                    token,
                    format!("invalid number format: {s}"),
                )),
            },
            Value::Bool(b) => {
                if *b {
                    Ok(Value::Number(1.0))
                } else {
                    Ok(Value::Number(0.0))
                }
            }
            Value::Nil => Ok(Value::Number(0.0)),
            _ => Err(RuntimeError::new(
                token,
                "cannot convert this type to a number",
            )),
        }
    }

    fn name(&self) -> &str {
        "to_number"
    }
}

pub struct InfoFn;

impl KyroCallable for InfoFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let class = Rc::new(KyroClass {
            name: "LanguageInfo".to_string(),
            superclass: None,
            methods: HashMap::new(),
            doc: None,
        });

        let mut fields = HashMap::new();
        fields.insert("language".to_string(), Value::String("kyro".to_string()));
        fields.insert("version".to_string(), Value::String(VERSION.to_string()));

        let instance = KyroInstance { class, fields };
        Ok(Value::Instance(Rc::new(RefCell::new(instance))))
    }

    fn name(&self) -> &str {
        "info"
    }

    fn doc(&self) -> Option<&str> {
        return Some("info about the language");
    }
}
