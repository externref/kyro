use crate::{
    interpreter::{interpreter::Interpreter, runtime_error::RuntimeError},
    parser::{parser::Parser, scanner::Scanner},
};
use std::io::Write;

pub struct Kyro {
    had_error: bool,
    interpreter: Interpreter,
}
impl Kyro {
    pub fn new() -> Kyro {
        Kyro {
            had_error: false,
            interpreter: Interpreter::new(),
        }
    }
    pub fn run_file(&mut self, path: &str) -> std::io::Result<()> {
        let file_content = std::fs::read_to_string(path)?;
        self.run(&file_content);
        Ok(())
    }
    pub fn run_prompt(&mut self) -> std::io::Result<()> {
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
        let scanner = Scanner::new(src.to_string(), self);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        for stmt in statements {
            if let Err(error) = self.interpreter.execute(&stmt) {
                match error {
                    RuntimeError::Error { token, message } => {
                        eprintln!("[line {}] RuntimeError: {}", token.line, message);
                    }
                    RuntimeError::Return(_) => {}
                }
            }
        }
    }
    fn report(&mut self, line: usize, r#where: &str, msg: &str) {
        println!("[line {}] Error {}: {}", line, r#where, msg);
        self.had_error = true
    }
    pub fn error(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }
}
