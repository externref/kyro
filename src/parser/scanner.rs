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

use crate::parser::tokens::{Literal, Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub errors: Vec<(usize, String, String)>,
}

impl Scanner {
    pub fn new(source: String, start_line: usize) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: start_line,
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
            '&' => self.add_token(TokenType::Ampersand),
            '|' => self.add_token(TokenType::Pipe),
            '^' => self.add_token(TokenType::Caret),
            '~' => self.add_token(TokenType::Tilde),

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
                } else if self.matches('<') {
                    TokenType::LessLess
                } else {
                    TokenType::Less
                };
                self.add_token(t);
            }

            '>' => {
                let t = if self.matches('=') {
                    TokenType::GreaterEqual
                } else if self.matches('>') {
                    TokenType::GreaterGreater
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
        let token_type = Self::keyword(&text).unwrap_or(TokenType::Identifier);
        self.add_token_with_text(token_type, text, None);
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
        self.add_token_with_text(TokenType::Number, text, Some(Literal::Number(value)));
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
        let text: String = self.source[self.start..self.current].iter().collect();
        self.add_token_with_text(TokenType::String, text, Some(Literal::String(value)));
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
        let text: String = self.source[self.start..self.current].iter().collect();
        self.add_token_with_text(token_type, text, None);
    }

    fn add_token_with_text(
        &mut self,
        token_type: TokenType,
        text: String,
        literal: Option<Literal>,
    ) {
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn keyword(text: &str) -> Option<TokenType> {
        match text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fn" => Some(TokenType::Fn),
            "if" => Some(TokenType::If),
            "in" => Some(TokenType::In),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "echo" => Some(TokenType::Echo),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            "try" => Some(TokenType::Try),
            "catch" => Some(TokenType::Catch),
            "throw" => Some(TokenType::Throw),
            "break" => Some(TokenType::Break),
            "continue" => Some(TokenType::Continue),
            _ => None,
        }
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
