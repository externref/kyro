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

pub mod ffi;
pub mod fs;
pub mod io;
pub mod os;
pub mod time;
pub mod util;

use crate::{
    interpreter::{
        callable::KyroCallable, class::KyroClass, environment::Environment, instance::KyroInstance,
        interpreter::Interpreter, resolver::Resolver, runtime_error::RuntimeError, value::Value,
    },
    parser::{
        parser::Parser,
        scanner::Scanner,
        tokens::{Token, TokenType},
    },
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
        let first_arg = arguments.into_iter().next().unwrap();
        let filename = match first_arg {
            Value::String(s) => s,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Argument to use() must be a string.",
                    Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                ));
            }
        };

        if let Some(cached_module) = interpreter.modules.get(&filename) {
            return Ok(cached_module.clone());
        }

        let module_instance = if filename == "os" || filename == "std:os" {
            os::get_module()
        } else if filename == "io" || filename == "std:io" {
            io::get_module()
        } else if filename == "time" || filename == "std:time" {
            time::get_module()
        } else if filename == "fs" || filename == "std:fs" {
            fs::get_module()
        } else if filename == "util" || filename == "std:util" {
            util::get_module()
        } else if filename == "ffi" || filename == "std:ffi" {
            ffi::get_module()
        } else {
            let resolved_filename = if filename.starts_with("lib:") {
                let lib_name = &filename[4..];
                let kyro_home = std::env::var("KYRO_HOME").unwrap_or_else(|_| ".".to_string());

                format!("{}/lib/{}.kyro", kyro_home, lib_name)
            } else {
                filename.clone()
            };

            let file_content = match std::fs::read_to_string(&resolved_filename) {
                Ok(content) => content,
                Err(e) => {
                    return Err(interpreter.raise_error(
                        "ValueError",
                        &format!("Failed to load module file '{resolved_filename}': {e}"),
                        Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                    ));
                }
            };

            let scanner = Scanner::new(file_content.clone(), 1);
            let (tokens, scanner_errors) = scanner.scan_tokens();
            if !scanner_errors.is_empty() {
                for (line, msg, lex) in scanner_errors {
                    report_module_error(&file_content, &resolved_filename, line, &lex, &msg);
                }
                return Err(interpreter.raise_error(
                    "TypeError",
                    &format!(
                        "Lexical syntax errors found inside imported module '{resolved_filename}'."
                    ),
                    Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                ));
            }

            let mut parser = Parser::new(tokens, interpreter.next_id);
            let statements = parser.parse();
            interpreter.next_id = parser.get_next_id_counter();

            if !parser.errors.is_empty() {
                for (token, message) in parser.errors {
                    report_module_error(
                        &file_content,
                        &resolved_filename,
                        token.line,
                        &token.lexeme,
                        &message,
                    );
                }
                return Err(interpreter.raise_error(
                    "TypeError",
                    &format!("Parsing errors found inside imported module '{resolved_filename}'."),
                    Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                ));
            }

            let module_env = Rc::new(RefCell::new(Environment::from_enclosing(
                interpreter.environment.clone(),
            )));

            module_env
                .borrow_mut()
                .define("__name__".to_string(), Value::String(filename.clone()));

            let previous_env = std::mem::replace(&mut interpreter.environment, module_env.clone());

            let mut resolver = Resolver::new(interpreter);
            let resolve_success = resolver.resolve(&statements);

            if !resolve_success {
                for (token, message) in resolver.errors {
                    report_module_error(
                        &file_content,
                        &resolved_filename,
                        token.line,
                        &token.lexeme,
                        &message,
                    );
                }
                interpreter.environment = previous_env;
                return Err(interpreter.raise_error(
                    "TypeError",
                    &format!("Static analysis resolution failed inside imported module '{resolved_filename}'."),
                    Token::new(TokenType::Identifier, "use".to_string(), None, 0),
                ));
            }

            for stmt in statements {
                if let Err(e) = interpreter.execute(&stmt) {
                    interpreter.environment = previous_env;
                    match e {
                        RuntimeError::Error { token, value } => {
                            report_module_error(
                                &file_content,
                                &resolved_filename,
                                token.line,
                                &token.lexeme,
                                &value.to_string(),
                            );
                            return Err(interpreter.raise_error(
                                "TypeError",
                                &format!(
                                    "Runtime error inside imported module '{resolved_filename}'."
                                ),
                                Token::new(TokenType::Identifier, "use".to_string(), None, 0),
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
                doc: None,
            });

            Value::Instance(Rc::new(RefCell::new(KyroInstance {
                class,
                fields: module_values,
            })))
        };

        interpreter
            .modules
            .insert(filename.clone(), module_instance.clone());
        Ok(module_instance)
    }

    fn name(&self) -> &str {
        "use"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["module".to_string()]
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

pub struct DirFn;

impl KyroCallable for DirFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let first_arg = arguments.into_iter().next().unwrap();
        let mut names: Vec<String> = Vec::new();

        match &first_arg {
            Value::Instance(instance) => {
                let inst = instance.borrow();
                for key in inst.fields.keys() {
                    names.push(key.clone());
                }
                let mut current_class = Some(inst.class.clone());
                while let Some(cls) = current_class {
                    for key in cls.methods.keys() {
                        names.push(key.clone());
                    }
                    current_class = cls.superclass.clone();
                }
            }
            Value::Class(class) => {
                let mut current_class = Some(class.clone());
                while let Some(cls) = current_class {
                    for key in cls.methods.keys() {
                        names.push(key.clone());
                    }
                    current_class = cls.superclass.clone();
                }
                names.push("__name__".to_string());
            }
            Value::Callable(_) => {
                names.push("__name__".to_string());
            }
            Value::List(_) => {
                for method in &["len", "push", "pop"] {
                    names.push(method.to_string());
                }
            }
            Value::Dict(_) => {
                for method in &["len", "keys", "remove"] {
                    names.push(method.to_string());
                }
            }
            Value::String(_) => {
                for method in &["len", "slice", "split"] {
                    names.push(method.to_string());
                }
            }
            Value::Number(_) => {
                for method in &["floor", "ceil", "round", "abs", "to_string"] {
                    names.push(method.to_string());
                }
            }
            _ => {}
        }

        names.sort();
        names.dedup();

        let deduped_vals: Vec<Value> = names.into_iter().map(Value::String).collect();
        Ok(Value::List(Rc::new(RefCell::new(deduped_vals))))
    }

    fn name(&self) -> &str {
        "dir"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["item".to_string()]
    }
}

pub struct IdFn;

impl KyroCallable for IdFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let first_arg = arguments.into_iter().next().unwrap();
        let address = match &first_arg {
            Value::Instance(instance) => Rc::as_ptr(instance) as usize,
            Value::List(list) => Rc::as_ptr(list) as usize,
            Value::Dict(dict) => Rc::as_ptr(dict) as usize,
            Value::Class(class) => Rc::as_ptr(class) as usize,
            Value::Callable(callable) => Rc::as_ptr(callable) as *const () as usize,
            Value::String(s) => s.as_ptr() as usize,
            val => val as *const Value as usize,
        };
        Ok(Value::Number(address as f64))
    }

    fn name(&self) -> &str {
        "id"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["item".to_string()]
    }
}

pub struct IsInstanceFn;

impl KyroCallable for IsInstanceFn {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let mut args_iter = arguments.into_iter();
        let first_arg = args_iter.next().unwrap();
        let second_arg = args_iter.next().unwrap();

        let target_class = match &second_arg {
            Value::Class(cls) => cls,
            _ => {
                return Err(interpreter.raise_error(
                    "TypeError",
                    "Second argument to is_instance() must be a class.",
                    Token::new(TokenType::Identifier, "is_instance".to_string(), None, 0),
                ));
            }
        };

        let instance = match &first_arg {
            Value::Instance(inst) => inst,
            _ => return Ok(Value::Bool(false)),
        };

        let mut current_class = Some(instance.borrow().class.clone());
        while let Some(cls) = current_class {
            if Rc::ptr_eq(&cls, target_class) {
                return Ok(Value::Bool(true));
            }
            current_class = cls.superclass.clone();
        }

        Ok(Value::Bool(false))
    }

    fn name(&self) -> &str {
        "is_instance"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["item".to_string(), "class".to_string()]
    }
}
