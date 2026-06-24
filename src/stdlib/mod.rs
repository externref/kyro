pub mod core;
pub mod fs;
pub mod io;
pub mod time;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, class::KyroClass, environment::Environment, instance::KyroInstance,
    interpreter::Interpreter, runtime_error::RuntimeError, value::Value,
};
use crate::parser::parser::Parser;
use crate::parser::resolver::Resolver;
use crate::parser::scanner::Scanner;
use crate::parser::tokens::{Token, TokenType};

use core::{InfoFn, ToNumber, ToStringFn};
use io::{Input, Print, Println};
use time::{Clock, Format, Now};
pub struct Use;

impl KyroCallable for Use {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let arg_val = &arguments[0];
        let filename = match arg_val {
            Value::String(s) => s,
            _ => {
                return Err(RuntimeError::new(
                    Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                    "Argument to use() must be a string.",
                ));
            }
        };

        if filename == "std:core" || filename == "core" {
            let class = Rc::new(KyroClass {
                name: "core".to_string(),
                superclass: None,
                methods: HashMap::new(),
            });
            let mut fields = HashMap::new();
            fields.insert("version".to_string(), Value::String("0.1.0".to_string()));
            fields.insert(
                "to_string".to_string(),
                Value::Callable(Rc::new(ToStringFn)),
            );
            fields.insert("to_number".to_string(), Value::Callable(Rc::new(ToNumber)));
            fields.insert("info".to_string(), Value::Callable(Rc::new(InfoFn)));

            let instance = KyroInstance { class, fields };
            return Ok(Value::Instance(Rc::new(RefCell::new(instance))));
        }

        if filename == "io" || filename == "std:io" {
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
            return Ok(Value::Instance(Rc::new(RefCell::new(instance))));
        }

        if filename == "time" || filename == "std:time" {
            let class = Rc::new(KyroClass {
                name: "time".to_string(),
                superclass: None,
                methods: HashMap::new(),
            });
            let mut fields = HashMap::new();
            fields.insert("clock".to_string(), Value::Callable(Rc::new(Clock)));
            fields.insert("now".to_string(), Value::Callable(Rc::new(Now)));
            fields.insert("format".to_string(), Value::Callable(Rc::new(Format)));

            let instance = KyroInstance { class, fields };
            return Ok(Value::Instance(Rc::new(RefCell::new(instance))));
        }
        if filename == "fs" || filename == "std:fs" {
            let class = Rc::new(KyroClass {
                name: "fs".to_string(),
                superclass: None,
                methods: HashMap::new(),
            });
            let mut fields = HashMap::new();
            fields.insert(
                "read_file".to_string(),
                Value::Callable(Rc::new(fs::ReadFile)),
            );
            fields.insert(
                "write_file".to_string(),
                Value::Callable(Rc::new(fs::WriteFile)),
            );
            fields.insert("exists".to_string(), Value::Callable(Rc::new(fs::Exists)));
            fields.insert(
                "remove_file".to_string(),
                Value::Callable(Rc::new(fs::RemoveFile)),
            );

            let instance = KyroInstance { class, fields };
            return Ok(Value::Instance(Rc::new(RefCell::new(instance))));
        }

        let file_content = match std::fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                return Err(RuntimeError::new(
                    Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                    format!("Failed to load module file '{filename}': {e}"),
                ));
            }
        };

        let scanner = Scanner::new(file_content);
        let (tokens, scanner_errors) = scanner.scan_tokens();
        if !scanner_errors.is_empty() {
            return Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                format!("Lexical syntax errors found inside imported module '{filename}'."),
            ));
        }

        let mut parser = Parser::new(tokens, interpreter.next_id);
        let statements = parser.parse();
        interpreter.next_id = parser.get_next_id_counter();

        let module_env = Rc::new(RefCell::new(Environment::from_enclosing(
            interpreter.environment.clone(),
        )));

        let previous_env = std::mem::replace(&mut interpreter.environment, module_env.clone());

        let mut resolver = Resolver::new(interpreter);
        let resolve_success = resolver.resolve(&statements);

        if !resolve_success {
            interpreter.environment = previous_env;
            return Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                format!("Static analysis resolution failed inside imported module '{filename}'."),
            ));
        }

        for stmt in statements {
            if let Err(e) = interpreter.execute(&stmt) {
                interpreter.environment = previous_env;
                return Err(e);
            }
        }

        let module_values = module_env.borrow().get_values();

        interpreter.environment = previous_env;

        let class = Rc::new(KyroClass {
            name: filename.to_string(),
            superclass: None,
            methods: HashMap::new(),
        });

        let instance = KyroInstance {
            class,
            fields: module_values,
        };

        Ok(Value::Instance(Rc::new(RefCell::new(instance))))
    }

    fn name(&self) -> &str {
        "use"
    }
}
