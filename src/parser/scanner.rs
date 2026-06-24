use crate::parser::tokens::{Literal, Token, TokenType};
use std::collections::HashMap;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
    pub errors: Vec<(usize, String, String)>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fn", TokenType::Fn);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("echo", TokenType::Echo);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);
        keywords.insert("try", TokenType::Try);
        keywords.insert("catch", TokenType::Catch);
        keywords.insert("throw", TokenType::Throw);
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(mut self) -> (Vec<Token>, Vec<(usize, String, String)>) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
        (self.tokens, self.errors)
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ':' => self.add_token(TokenType::Colon),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => {
                let t = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t);
            }

            '=' => {
                let t = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t);
            }

            '<' => {
                let t = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t);
            }

            '>' => {
                let t = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t);
            }

            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => {}

            '\n' => {
                self.line += 1;
            }

            '"' => self.string(),

            _ => {
                if Self::is_digit(c) {
                    self.number();
                } else if Self::is_alpha(c) {
                    self.identifier();
                } else {
                    let err_char = c.to_string();
                    self.error(self.line, "Unexpected character.", &err_char);
                }
            }
        }
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = self
            .keywords
            .get(text.as_str())
            .cloned()
            .unwrap_or(TokenType::Identifier);
        self.add_token(token_type);
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = text.parse().unwrap();
        self.add_token_literal(TokenType::Number, Some(Literal::Number(value)));
    }

    fn string(&mut self) {
        let mut value = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            let c = self.advance();
            if c == '\n' {
                self.line += 1;
                value.push(c);
            } else if c == '\\' {
                if !self.is_at_end() {
                    let next_c = self.advance();
                    match next_c {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        _ => {
                            value.push('\\');
                            value.push(next_c);
                        }
                    }
                } else {
                    value.push('\\');
                }
            } else {
                value.push(c);
            }
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.", "\"");
            return;
        }

        self.advance();
        self.add_token_literal(TokenType::String, Some(Literal::String(value)));
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn error(&mut self, line: usize, message: &str, lexeme: &str) {
        self.errors
            .push((line, message.to_string(), lexeme.to_string()));
    }
}
