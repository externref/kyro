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

use io::{Input, Print, Println};
use time::Clock;

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
                return Err(RuntimeError::Error {
                    token: Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                    message: "Argument to use() must be a string.".to_string(),
                });
            }
        };
        if filename == "io" {
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

        if filename == "time" {
            let class = Rc::new(KyroClass {
                name: "time".to_string(),
                superclass: None,
                methods: HashMap::new(),
            });
            let mut fields = HashMap::new();
            fields.insert("clock".to_string(), Value::Callable(Rc::new(Clock)));

            let instance = KyroInstance { class, fields };
            return Ok(Value::Instance(Rc::new(RefCell::new(instance))));
        }

        let file_content = match std::fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                return Err(RuntimeError::Error {
                    token: Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                    message: format!("Failed to load module file '{filename}': {e}"),
                });
            }
        };
        let scanner = Scanner::new(file_content);
        let (tokens, scanner_error) = scanner.scan_tokens();
        if scanner_error {
            return Err(RuntimeError::Error {
                token: Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                message: format!(
                    "Lexical syntax errors found inside imported module '{filename}'."
                ),
            });
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
            return Err(RuntimeError::Error {
                token: Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                message: format!(
                    "Static analysis resolution failed inside imported module '{filename}'."
                ),
            });
        }
        for stmt in statements {
            if let Err(e) = interpreter.execute(&stmt) {
                // Restore scope before exiting
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
