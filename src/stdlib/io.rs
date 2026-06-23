use crate::interpreter::{
    callable::KyroCallable, interpreter::Interpreter, runtime_error::RuntimeError, value::Value,
};
use crate::parser::tokens::{Token, TokenType};
use std::io::{self, Write};

fn interpolate(format_str: &str, interpreter: &mut Interpreter) -> Result<String, RuntimeError> {
    let dummy_token = Token::new(TokenType::Identifier, "print".to_string(), None, 0);

    let mut result = String::new();
    let mut chars = format_str.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' && chars.peek() == Some(&'{') {
            chars.next();
            let mut var_name = String::new();
            let mut closed = false;

            while let Some(inner_c) = chars.next() {
                if inner_c == '}' {
                    closed = true;
                    break;
                }
                var_name.push(inner_c);
            }

            if !closed {
                return Err(RuntimeError::Error {
                    token: dummy_token.clone(),
                    message: "Unterminated interpolation bracket.".to_string(),
                });
            }

            let trimmed_name = var_name.trim();

            if let Some(val) = interpreter.environment.borrow().get(trimmed_name) {
                result.push_str(&val.to_string());
            } else {
                return Err(RuntimeError::Error {
                    token: dummy_token.clone(),
                    message: format!(
                        "Undefined variable '{}' inside format string.",
                        trimmed_name
                    ),
                });
            }
        } else {
            result.push(c);
        }
    }
    Ok(result)
}

pub struct Print;

impl KyroCallable for Print {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let format_val = &arguments[0];
        let result = match format_val {
            Value::String(s) => interpolate(s, interpreter)?,
            _ => format_val.to_string(),
        };

        print!("{}", result);
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
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let format_val = &arguments[0];
        let result = match format_val {
            Value::String(s) => interpolate(s, interpreter)?,
            _ => format_val.to_string(),
        };

        println!("{}", result);
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
            return Err(RuntimeError::Error {
                token: Token::new(TokenType::Identifier, "input".to_string(), None, 0),
                message: format!("Failed to flush stdout: {e}"),
            });
        }

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let trimmed = buffer
                    .trim_end_matches(|c| c == '\r' || c == '\n')
                    .to_string();
                Ok(Value::String(trimmed))
            }
            Err(e) => Err(RuntimeError::Error {
                token: Token::new(TokenType::Identifier, "input".to_string(), None, 0),
                message: format!("Failed to read input: {e}"),
            }),
        }
    }

    fn name(&self) -> &str {
        "input"
    }
}
