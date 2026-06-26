use super::{
    expr::Expr,
    stmt::Stmt,
    tokens::{Literal, Token, TokenType},
};
use crate::parser::scanner::Scanner;

#[derive(Debug)]
pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    next_id: usize,
    pub errors: Vec<(Token, String)>,
    pub module_doc: Option<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, start_id: usize) -> Self {
        Self {
            tokens,
            current: 0,
            next_id: start_id,
            errors: Vec::new(),
            module_doc: None,
        }
    }

    pub fn get_next_id_counter(&self) -> usize {
        self.next_id
    }

    fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        if !statements.is_empty() {
            if let Stmt::Expression(Expr::Literal(Literal::String(s))) = &statements[0] {
                self.module_doc = Some(s.clone());
                statements.remove(0);
            }
        }

        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let result = if self.match_token(&[TokenType::Fn]) {
            self.function("function")
        } else if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else if self.match_token(&[TokenType::Class]) {
            self.class_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(stmt) => Some(stmt),

            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn class_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?;

        let super_class = if self.match_token(&[TokenType::Less]) {
            self.consume(TokenType::Identifier, "Expect superclass name.")?;
            let id = self.get_next_id();
            Some(Expr::Variable {
                name: self.previous().clone(),
                id,
            })
        } else {
            None
        };

        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;
        let mut doc = None;
        if self.check(&TokenType::String) {
            let token = self.advance().clone();
            if let Some(Literal::String(s)) = &token.literal {
                doc = Some(s.clone());
                self.consume(TokenType::Semicolon, "Expect ';' after class docstring.")?;
            }
        }
        let mut methods = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;

        Ok(Stmt::Class {
            name,
            super_class,
            methods,
            doc,
        })
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", kind))?;

        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;

        let mut params = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                params.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let mut body = self.block()?;
        let mut doc = None;
        if !body.is_empty() {
            if let Stmt::Expression(Expr::Literal(Literal::String(s))) = &body[0] {
                doc = Some(s.clone());
                body.remove(0);
            }
        }

        Ok(Stmt::Function {
            name,
            params,
            body,
            doc,
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Echo]) {
            return self.echo_statement();
        }
        if self.match_token(&[TokenType::Break]) {
            return self.break_statement();
        }
        if self.match_token(&[TokenType::Continue]) {
            return self.continue_statement();
        }
        if self.match_token(&[TokenType::Try]) {
            return self.try_statement();
        }
        if self.match_token(&[TokenType::Throw]) {
            return self.throw_statement();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }
        self.expression_statement()
    }

    fn try_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftBrace, "Expect '{' before try block.")?;
        let try_branch = Box::new(Stmt::Block(self.block()?));

        self.consume(TokenType::Catch, "Expect 'catch' after try block.")?;
        self.consume(TokenType::LeftParen, "Expect '(' after 'catch'.")?;
        let exception_var =
            self.consume(TokenType::Identifier, "Expect exception variable name.")?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after exception variable.",
        )?;

        self.consume(TokenType::LeftBrace, "Expect '{' before catch block.")?;
        let catch_branch = Box::new(Stmt::Block(self.block()?));

        Ok(Stmt::TryCatch {
            try_branch,
            exception_var,
            catch_branch,
        })
    }

    fn throw_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after throw value.")?;
        Ok(Stmt::Throw { keyword, value })
    }

    fn break_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        self.consume(TokenType::Semicolon, "Expect ';' after 'break'.")?;
        Ok(Stmt::Break { keyword })
    }

    fn continue_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        self.consume(TokenType::Semicolon, "Expect ';' after 'continue'.")?;
        Ok(Stmt::Continue { keyword })
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();

        let value = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::Return { keyword, value })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;

        let condition = self.expression()?;

        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.match_token(&[TokenType::Var]) {
            Some(Box::new(self.var_declaration()?))
        } else {
            Some(Box::new(self.expression_statement()?))
        };

        let condition = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let body = Box::new(self.statement()?);
        let cond = condition.unwrap_or(Expr::Literal(Literal::Bool(true)));

        Ok(Stmt::For {
            initializer,
            condition: cond,
            increment,
            body,
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;

        let condition = self.expression()?;

        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn echo_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Echo(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;

        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_token(&[TokenType::Equal]) {
            let value = self.assignment()?;

            if let Expr::Get { object, name } = expr {
                return Ok(Expr::Set {
                    object,
                    name,
                    value: Box::new(value),
                });
            }

            if let Expr::Variable {
                name: name_token,
                id: _,
            } = expr
            {
                let id = self.get_next_id();
                return Ok(Expr::Assign {
                    name: name_token,
                    value: Box::new(value),
                    id,
                });
            }

            if let Expr::Subscript {
                object,
                index,
                paren,
                id: _,
            } = expr
            {
                let id = self.get_next_id();
                return Ok(Expr::SubscriptAssign {
                    object,
                    index,
                    value: Box::new(value),
                    paren,
                    id,
                });
            }

            let prev_token = self.previous().clone();
            return Err(self.error(&prev_token, "Invalid assignment target."));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous().clone();

            let right = self.and()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let operator = self.previous().clone();

            let right = self.equality()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&[TokenType::Dot]) {
                let name = self.consume(TokenType::Identifier, "Expect property name.")?;
                expr = Expr::Get {
                    object: Box::new(expr),
                    name,
                };
            } else if self.match_token(&[TokenType::LeftBracket]) {
                let index = self.expression()?;
                let paren = self.consume(TokenType::RightBracket, "Expect ']' after index.")?;
                let id = self.get_next_id();
                expr = Expr::Subscript {
                    object: Box::new(expr),
                    index: Box::new(index),
                    paren,
                    id,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();

            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.call()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_token(&[TokenType::Number, TokenType::String]) {
            let token = self.previous().clone();
            if let Some(lit) = &token.literal {
                match lit {
                    Literal::String(s) => {
                        if s.contains("${") {
                            return self.parse_interpolation(s, &token);
                        } else {
                            return Ok(Expr::Literal(lit.clone()));
                        }
                    }
                    _ => return Ok(Expr::Literal(lit.clone())),
                }
            } else {
                return Err(self.error(&token, "Missing literal value"));
            }
        }

        if self.match_token(&[TokenType::Super]) {
            let keyword = self.previous().clone();
            self.consume(TokenType::Dot, "Expect '.' after 'super'.")?;
            let method = self.consume(TokenType::Identifier, "Expect superclass method name.")?;
            let id = self.get_next_id();
            return Ok(Expr::Super {
                keyword,
                method,
                id,
            });
        }

        if self.match_token(&[TokenType::This]) {
            let id = self.get_next_id();
            return Ok(Expr::This {
                keyword: self.previous().clone(),
                id,
            });
        }

        if self.match_token(&[TokenType::Identifier]) {
            let id = self.get_next_id();
            return Ok(Expr::Variable {
                name: self.previous().clone(),
                id,
            });
        }

        if self.match_token(&[TokenType::LeftBracket]) {
            let mut elements = Vec::new();
            if !self.check(&TokenType::RightBracket) {
                loop {
                    elements.push(self.expression()?);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightBracket, "Expect ']' after list elements.")?;
            let id = self.get_next_id();
            return Ok(Expr::List { elements, id });
        }

        if self.match_token(&[TokenType::LeftBrace]) {
            let mut entries = Vec::new();
            if !self.check(&TokenType::RightBrace) {
                loop {
                    let key = self.expression()?;
                    self.consume(TokenType::Colon, "Expect ':' after dictionary key.")?;
                    let value = self.expression()?;
                    entries.push((key, value));
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(
                TokenType::RightBrace,
                "Expect '}' after dictionary entries.",
            )?;
            let id = self.get_next_id();
            return Ok(Expr::Dict { entries, id });
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;

            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let peek_token = self.peek().clone();
        Err(self.error(&peek_token, "Expect expression."))
    }

    fn parse_interpolation(&mut self, format_str: &str, token: &Token) -> Result<Expr, ParseError> {
        let mut parts = Vec::new();
        let mut chars = format_str.chars().peekable();
        let mut current_literal = String::new();

        while let Some(c) = chars.next() {
            if c == '$' && chars.peek() == Some(&'{') {
                chars.next(); // Consume '{'

                if !current_literal.is_empty() {
                    parts.push(Expr::Literal(Literal::String(current_literal.clone())));
                    current_literal.clear();
                }

                let mut expr_str = String::new();
                let mut closed = false;

                while let Some(inner_c) = chars.next() {
                    if inner_c == '}' {
                        closed = true;
                        break;
                    }
                    expr_str.push(inner_c);
                }

                if !closed {
                    return Err(self.error(token, "Unterminated interpolation bracket."));
                }

                let scanner = Scanner::new(expr_str, token.line);
                let (tokens, scanner_errors) = scanner.scan_tokens();
                if !scanner_errors.is_empty() {
                    return Err(self.error(token, "Syntax error inside interpolation block."));
                }

                let mut sub_parser = Parser::new(tokens, self.next_id);
                let expr = sub_parser.expression()?;
                self.next_id = sub_parser.get_next_id_counter();

                if !sub_parser.errors.is_empty() {
                    return Err(self.error(token, "Parser error inside interpolation block."));
                }

                parts.push(expr);
            } else {
                current_literal.push(c);
            }
        }

        if !current_literal.is_empty() {
            parts.push(Expr::Literal(Literal::String(current_literal)));
        }

        let id = self.get_next_id();
        Ok(Expr::Interpolate { parts, id })
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == TokenType::Semicolon {
                return;
            }

            match self.peek().r#type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Echo
                | TokenType::Return
                | TokenType::Try
                | TokenType::Throw => return,

                _ => {}
            }
            self.advance();
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for r#type in types {
            if self.check(r#type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, r#type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(&r#type) {
            return Ok(self.advance().clone());
        }

        let peek_token = self.peek().clone();
        Err(self.error(&peek_token, message))
    }

    fn check(&self, r#type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().r#type == *r#type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&mut self, token: &Token, message: &str) -> ParseError {
        self.errors.push((token.clone(), message.to_string()));
        ParseError
    }
}
