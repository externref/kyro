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
    fields.insert("type_of".to_string(), Value::Callable(Rc::new(TypeOfFn)));
    fields.insert("range".to_string(), Value::Callable(Rc::new(RangeFn)));

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
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let str_val = interpreter.stringify(&arguments[0]);
        Ok(Value::String(str_val))
    }

    fn name(&self) -> &str {
        "to_string"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["value".to_string()]
    }
}

pub struct ToNumber;

impl KyroCallable for ToNumber {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let token = Token::new(TokenType::Identifier, "to_number".to_string(), None, 0);
        match &arguments[0] {
            Value::Number(n) => Ok(Value::Number(*n)),
            Value::String(s) => match s.trim().parse::<f64>() {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Err(interpreter.raise_error(
                    "ValueError",
                    &format!("invalid number format: {s}"),
                    token,
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
            _ => Err(interpreter.raise_error(
                "TypeError",
                "cannot convert this type to a number",
                token,
            )),
        }
    }

    fn name(&self) -> &str {
        "to_number"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["value".to_string()]
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
        Some("info about the language")
    }
}

pub struct TypeOfFn;

impl KyroCallable for TypeOfFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let class_name = match &arguments[0] {
            Value::Nil => "Nil",
            Value::Bool(_) => "Bool",
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::List(_) => "List",
            Value::Dict(_) => "Dict",
            Value::Class(_) => "Class",
            Value::Instance(inst) => return Ok(Value::Class(inst.borrow().class.clone())),
            Value::Callable(_) => "Callable",
        };

        let global_cls = interpreter
            .environment
            .borrow()
            .get(class_name)
            .unwrap_or(Value::Nil);

        Ok(global_cls)
    }

    fn name(&self) -> &str {
        "type_of"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["value".to_string()]
    }
}

pub struct RangeFn;

impl KyroCallable for RangeFn {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let start = match arguments[0] {
            Value::Number(n) => n,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "First argument 'start' must be a number.",
                    Token::new(TokenType::Identifier, "range".to_string(), None, 0),
                ));
            }
        };

        let end = match arguments[1] {
            Value::Number(n) => n,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Second argument 'end' must be a number.",
                    Token::new(TokenType::Identifier, "range".to_string(), None, 0),
                ));
            }
        };

        let step = match arguments[2] {
            Value::Number(n) => n,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Third argument 'step' must be a number.",
                    Token::new(TokenType::Identifier, "range".to_string(), None, 0),
                ));
            }
        };

        if step == 0.0 {
            return Err(interpreter.raise_error(
                "ValueError",
                "Range step size cannot be zero.",
                Token::new(TokenType::Identifier, "range".to_string(), None, 0),
            ));
        }

        let mut values = Vec::new();
        if step > 0.0 {
            let mut curr = start;
            while curr < end {
                values.push(Value::Number(curr));
                curr += step;
            }
        } else {
            let mut curr = start;
            while curr > end {
                values.push(Value::Number(curr));
                curr += step;
            }
        }

        Ok(Value::List(Rc::new(RefCell::new(values))))
    }

    fn name(&self) -> &str {
        "range"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["start".to_string(), "end".to_string(), "step".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "step" {
            Some(Ok(Value::Number(1.0)))
        } else {
            None
        }
    }
}
