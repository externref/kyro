use super::{
    expr::Expr,
    stmt::Stmt,
    tokens::{Literal, Token, TokenType},
};

#[derive(Debug)]
pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    next_id: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, start_id: usize) -> Self {
        Self {
            tokens,
            current: 0,
            next_id: start_id,
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

        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let result = if self.match_token(&[TokenType::Fun]) {
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

        let mut methods = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;

        Ok(Stmt::Class {
            name,
            super_class,
            methods,
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
        let body = self.block()?;
        Ok(Stmt::Function { name, params, body })
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
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
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

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(increment)]);
        }

        let condition = condition.unwrap_or(Expr::Literal(Literal::Bool(true)));

        body = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![initializer, body]);
        }

        Ok(body)
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
            return Err(self.error(&self.previous(), "Invalid assignment target."));
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
            let token = self.previous();

            return Ok(Expr::Literal(
                token
                    .literal
                    .clone()
                    .ok_or_else(|| self.error(token, "Missing literal value"))?,
            ));
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

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == TokenType::Semicolon {
                return;
            }

            match self.peek().r#type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Echo 
                | TokenType::Return => return,

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
        Err(self.error(self.peek(), message))
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

    fn error(&self, token: &Token, message: &str) -> ParseError {
        if token.r#type == TokenType::Eof {
            eprintln!("[line {}] Error at end: {}", token.line, message);
        } else {
            eprintln!(
                "[line {}] Error at '{}': {}",
                token.line, token.lexeme, message
            );
        }
        ParseError
    }
}
