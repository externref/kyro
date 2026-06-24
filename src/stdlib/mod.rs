pub mod fs;
pub mod io;
pub mod os;
pub mod time;
pub mod util;

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

        if filename == "os" || filename == "std:os" {
            return Ok(os::get_module());
        }
        if filename == "io" || filename == "std:io" {
            return Ok(io::get_module());
        }
        if filename == "time" || filename == "std:time" {
            return Ok(time::get_module());
        }
        if filename == "fs" || filename == "std:fs" {
            return Ok(fs::get_module());
        }
        if filename == "util" || filename == "std:util" {
            return Ok(util::get_module());
        }

        let resolved_filename = if filename.starts_with("lib:") {
            let lib_name = &filename[4..];
            let kyro_home = std::env::var("KYRO_HOME")
                .unwrap_or_else(|_| ".".to_string());
            
            format!("{}/lib/{}.kyro", kyro_home, lib_name)
        } else {
            filename.clone()
        };

        let file_content = match std::fs::read_to_string(&resolved_filename) {
            Ok(content) => content,
            Err(e) => {
                return Err(RuntimeError::new(
                    Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                    format!("Failed to load module file '{resolved_filename}': {e}"),
                ));
            }
        };

        let scanner = Scanner::new(file_content.clone(), 1);
        let (tokens, scanner_errors) = scanner.scan_tokens();
        if !scanner_errors.is_empty() {
            for (line, msg, lex) in scanner_errors {
                report_module_error(&file_content, &resolved_filename, line, &lex, &msg);
            }
            return Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                format!("Lexical syntax errors found inside imported module '{resolved_filename}'."),
            ));
        }

        let mut parser = Parser::new(tokens, interpreter.next_id);
        let statements = parser.parse();
        interpreter.next_id = parser.get_next_id_counter();

        if !parser.errors.is_empty() {
            for (token, message) in parser.errors {
                report_module_error(&file_content, &resolved_filename, token.line, &token.lexeme, &message);
            }
            return Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                format!("Parsing errors found inside imported module '{resolved_filename}'."),
            ));
        }

        let module_env = Rc::new(RefCell::new(Environment::from_enclosing(
            interpreter.environment.clone(),
        )));

        let previous_env = std::mem::replace(&mut interpreter.environment, module_env.clone());

        let mut resolver = Resolver::new(interpreter);
        let resolve_success = resolver.resolve(&statements);

        if !resolve_success {
            for (token, message) in resolver.errors {
                report_module_error(&file_content, &resolved_filename, token.line, &token.lexeme, &message);
            }
            interpreter.environment = previous_env;
            return Err(RuntimeError::new(
                Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                format!("Static analysis resolution failed inside imported module '{resolved_filename}'."),
            ));
        }

        for stmt in statements {
            if let Err(e) = interpreter.execute(&stmt) {
                interpreter.environment = previous_env;
                match e {
                    RuntimeError::Error { token, value } => {
                        report_module_error(&file_content, &resolved_filename, token.line, &token.lexeme, &value.to_string());
                        return Err(RuntimeError::new(
                            Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                            format!("Runtime error inside imported module '{resolved_filename}'."),
                        ));
                    }
                    _ => return Err(e),
                }
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

fn report_module_error(source: &str, filename: &str, line: usize, lexeme: &str, msg: &str) {
    let lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
    eprintln!("\x1b[1;31merror\x1b[0m: {msg}");

    if line > 0 && line <= lines.len() {
        let line_content = &lines[line - 1];
        eprintln!("  --> {filename}:{line}:");
        eprintln!("     |");
        eprintln!("{:4} | {line_content}", line);

        let col = if lexeme.is_empty() {
            line_content.len()
        } else {
            line_content.find(lexeme).unwrap_or(0)
        };

        let padding = " ".repeat(col);
        let carets = "^".repeat(lexeme.len().max(1));

        eprintln!("     | {}\x1b[1;31m{}\x1b[0m", padding, carets);
        eprintln!("     |");
    }
    eprintln!();
}