use crate::{
    interpreter::{interpreter::Interpreter, runtime_error::RuntimeError},
    parser::{parser::Parser, resolver::Resolver, scanner::Scanner},
};
use std::io::Write;

pub struct Kyro {
    had_error: bool,
    interpreter: Interpreter,
    source_lines: Vec<String>,
}

impl Kyro {
    pub fn new() -> Kyro {
        let mut kyro = Kyro {
            had_error: false,
            interpreter: Interpreter::new(),
            source_lines: Vec::new(),
        };
        kyro.bootstrap();
        kyro
    }

    fn bootstrap(&mut self) {
        let boot_code = r#"
            class Exception {
                init(message = "") {
                    this.message = message;
                }
                __str__() {
                    return this.__class__.__name__ + ": " + this.message;
                }
                to_string() {
                    return this.__str__();
                }
            }
            class ValueError < Exception {}
            class AttributeError < Exception {}
            class TypeError < Exception {}
            class IndexError < Exception {}

            class List {}
            class Dict {}
            class Number {}
            class String {}
            class Bool {}
            class Nil {}
            class Callable {}
            class Class {}
        "#;

        let prev_lines = std::mem::take(&mut self.source_lines);

        let scanner = Scanner::new(boot_code.to_string(), 1);
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens, self.interpreter.next_id);
        let statements = parser.parse();
        self.interpreter.next_id = parser.get_next_id_counter();

        let mut resolver = Resolver::new(&mut self.interpreter);
        resolver.resolve(&statements);

        for stmt in &statements {
            let _ = self.interpreter.execute(stmt);
        }

        self.source_lines = prev_lines;
    }

    pub fn run_file(&mut self, path: &str) -> std::io::Result<()> {
        let file_content = std::fs::read_to_string(path)?;
        self.run(&file_content);
        if self.had_error {
            std::process::exit(70);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> std::io::Result<()> {
        println!("kyro interactive prompt. press ctrl+d to exit.");

        let stdin = std::io::stdin();
        loop {
            print!("> ");
            std::io::stdout().flush()?;
            let mut line = String::new();
            let bytes_read = stdin.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            self.run(line.trim_end());
        }
        Ok(())
    }

    fn run(&mut self, src: &str) {
        self.source_lines = src.lines().map(|s| s.to_string()).collect();
        self.had_error = false;

        let scanner = Scanner::new(src.to_string(), 1);
        let (tokens, scanner_errors) = scanner.scan_tokens();
        if !scanner_errors.is_empty() {
            for (line, msg, lex) in scanner_errors {
                self.report_context_error(line, &lex, &msg);
            }
            return;
        }

        let mut parser = Parser::new(tokens, self.interpreter.next_id);
        let statements = parser.parse();
        self.interpreter.next_id = parser.get_next_id_counter();

        if !parser.errors.is_empty() {
            for (token, message) in parser.errors {
                self.report_context_error(token.line, &token.lexeme, &message);
            }
            return;
        }

        let mut resolver = Resolver::new(&mut self.interpreter);
        resolver.resolve(&statements);

        if !resolver.errors.is_empty() {
            for (token, message) in resolver.errors {
                self.report_context_error(token.line, &token.lexeme, &message);
            }
            return;
        }

        for stmt in statements {
            if let Err(error) = self.interpreter.execute(&stmt) {
                match error {
                    RuntimeError::Error { token, value } => {
                        let error_message = self.interpreter.stringify(&value);
                        self.report_context_error(token.line, &token.lexeme, &error_message);
                    }
                    RuntimeError::Return(_) => {}
                    RuntimeError::Break | RuntimeError::Continue => {}
                }
                break;
            }
        }
    }

    fn report_context_error(&mut self, line: usize, lexeme: &str, msg: &str) {
        eprintln!("\x1b[1;31merror\x1b[0m: {msg}");

        if line > 0 && line <= self.source_lines.len() {
            let line_content = &self.source_lines[line - 1];
            eprintln!("  --> line {line}:");
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
        self.had_error = true;
    }
}
