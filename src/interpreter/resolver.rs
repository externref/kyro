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

use crate::{
    interpreter::interpreter::Interpreter,
    parser::{
        expr::{Argument, Expr, ExprVisitor},
        stmt::{Parameter, Stmt},
        tokens::Token,
    },
};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Clone, Copy, PartialEq)]
enum ClassType {
    None,
    Class,
    Subclass,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
    pub errors: Vec<(Token, String)>,
    is_inside_loop: bool,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
            errors: Vec::new(),
            is_inside_loop: false,
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> bool {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
        self.errors.is_empty()
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block(statements) => {
                self.begin_scope();
                self.resolve_block(statements);
                self.end_scope();
            }
            Stmt::Var { name, initializer } => {
                self.declare(name);
                if let Some(init) = initializer {
                    self.resolve_expr(init);
                }
                self.define(name);
            }
            Stmt::VarDestructure {
                names,
                is_dict: _,
                initializer,
            } => {
                self.resolve_expr(initializer);
                for name in names {
                    self.declare(name);
                    self.define(name);
                }
            }
            Stmt::Function {
                name,
                params,
                body,
                doc: _,
            } => {
                self.declare(name);
                self.define(name);
                self.resolve_function(params, body, FunctionType::Function);
            }
            Stmt::Expression(expr) => {
                self.resolve_expr(expr);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition);
                self.resolve_stmt(then_branch);
                if let Some(else_b) = else_branch {
                    self.resolve_stmt(else_b);
                }
            }
            Stmt::Echo(expr) => {
                self.resolve_expr(expr);
            }
            Stmt::Return { keyword, value } => {
                if self.current_function == FunctionType::None {
                    self.error(keyword, "Can't return from top-level code.");
                }
                if let Some(val) = value {
                    if self.current_function == FunctionType::Initializer {
                        self.error(keyword, "Can't return a value from an initializer.");
                    }
                    self.resolve_expr(val);
                }
            }
            Stmt::While { condition, body } => {
                let enclosing_loop = self.is_inside_loop;
                self.is_inside_loop = true;
                self.resolve_expr(condition);
                self.resolve_stmt(body);
                self.is_inside_loop = enclosing_loop;
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                let enclosing_loop = self.is_inside_loop;
                self.is_inside_loop = true;

                self.begin_scope();

                if let Some(init) = initializer {
                    self.resolve_stmt(init);
                }
                self.resolve_expr(condition);
                if let Some(inc) = increment {
                    self.resolve_expr(inc);
                }
                self.resolve_stmt(body);

                self.end_scope();

                self.is_inside_loop = enclosing_loop;
            }
            Stmt::ForIn {
                var_name,
                collection,
                body,
            } => {
                let enclosing_loop = self.is_inside_loop;
                self.is_inside_loop = true;

                self.resolve_expr(collection);

                self.begin_scope();
                self.declare(var_name);
                self.define(var_name);
                self.resolve_stmt(body);
                self.end_scope();

                self.is_inside_loop = enclosing_loop;
            }
            Stmt::Break { keyword } => {
                if !self.is_inside_loop {
                    self.error(keyword, "Can't use 'break' outside of a loop.");
                }
            }
            Stmt::Continue { keyword } => {
                if !self.is_inside_loop {
                    self.error(keyword, "Can't use 'continue' outside of a loop.");
                }
            }
            Stmt::Class {
                name,
                super_class,
                methods,
                doc: _,
            } => {
                let enclosing_class = self.current_class;
                self.current_class = ClassType::Class;

                self.declare(name);
                self.define(name);

                if let Some(super_expr) = super_class {
                    if let Expr::Variable {
                        name: super_name, ..
                    } = super_expr
                    {
                        if name.lexeme == super_name.lexeme {
                            self.error(super_name, "A class can't inherit from itself.");
                        }
                    }

                    self.current_class = ClassType::Subclass;
                    self.resolve_expr(super_expr);

                    self.begin_scope();
                    if let Some(scope) = self.scopes.last_mut() {
                        scope.insert("super".to_string(), true);
                    }
                }

                self.begin_scope();
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert("this".to_string(), true);
                }

                for method in methods {
                    if let Stmt::Function {
                        name: mname,
                        params,
                        body,
                        doc: _,
                    } = method
                    {
                        let declaration_type = if mname.lexeme == "__init__" {
                            FunctionType::Initializer
                        } else {
                            FunctionType::Method
                        };
                        self.resolve_function(params, body, declaration_type);
                    }
                }

                self.end_scope();

                if super_class.is_some() {
                    self.end_scope();
                }

                self.current_class = enclosing_class;
            }
            Stmt::TryCatch {
                try_branch,
                exception_var,
                catch_branch,
            } => {
                self.resolve_stmt(try_branch);

                self.begin_scope();
                self.declare(exception_var);
                self.define(exception_var);
                self.resolve_stmt(catch_branch);
                self.end_scope();
            }
            Stmt::Throw { keyword: _, value } => {
                self.resolve_expr(value);
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self);
    }

    fn resolve_block(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_function(&mut self, params: &[Parameter], body: &[Stmt], func_type: FunctionType) {
        let enclosing_function = self.current_function;
        self.current_function = func_type;

        self.begin_scope();
        for param in params {
            if let Some(default_val) = &param.default_value {
                self.resolve_expr(default_val);
            }
            self.declare(&param.name);
            self.define(&param.name);
        }
        self.resolve_block(body);
        self.end_scope();

        self.current_function = enclosing_function;
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        let already_exists = self
            .scopes
            .last()
            .map_or(false, |scope| scope.contains_key(&name.lexeme));

        if already_exists {
            self.error(name, "Already a variable with this name in this scope.");
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, name: &Token, id: usize) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(id, i);
                return;
            }
        }
    }

    fn error(&mut self, token: &Token, message: &str) {
        self.errors.push((token.clone(), message.to_string()));
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
    fn visit_variable(&mut self, name: &Token, id: usize) {
        if let Some(scope) = self.scopes.last() {
            if scope.get(&name.lexeme) == Some(&false) {
                self.error(name, "Can't read local variable in its own initializer.");
            }
        }
        self.resolve_local(name, id);
    }

    fn visit_assign(&mut self, name: &Token, value: &Expr, id: usize) {
        self.resolve_expr(value);
        self.resolve_local(name, id);
    }

    fn visit_binary(&mut self, left: &Expr, _operator: &Token, right: &Expr) {
        self.resolve_expr(left);
        self.resolve_expr(right);
    }

    fn visit_call(&mut self, callee: &Expr, _paren: &Token, arguments: &[Argument]) {
        self.resolve_expr(callee);
        for arg in arguments {
            match arg {
                Argument::Positional(expr) => self.resolve_expr(expr),
                Argument::Keyword { name: _, value } => self.resolve_expr(value),
            }
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) {
        self.resolve_expr(expr);
    }

    fn visit_literal(&mut self, _value: &crate::parser::tokens::Literal) {}

    fn visit_logical(&mut self, left: &Expr, _operator: &Token, right: &Expr) {
        self.resolve_expr(left);
        self.resolve_expr(right);
    }

    fn visit_unary(&mut self, _operator: &Token, right: &Expr) {
        self.resolve_expr(right);
    }

    fn visit_get(&mut self, object: &Expr, _name: &Token) {
        self.resolve_expr(object);
    }

    fn visit_set(&mut self, object: &Expr, _name: &Token, value: &Expr) {
        self.resolve_expr(value);
        self.resolve_expr(object);
    }

    fn visit_this(&mut self, keyword: &Token, id: usize) {
        if self.current_class == ClassType::None {
            self.error(keyword, "Can't use 'this' outside of a class.");
            return;
        }
        self.resolve_local(keyword, id);
    }

    fn visit_super(&mut self, keyword: &Token, _method: &Token, id: usize) {
        if self.current_class == ClassType::None {
            self.error(keyword, "Can't use 'super' outside of a class.");
        } else if self.current_class != ClassType::Subclass {
            self.error(keyword, "Can't use 'super' in a class with no superclass.");
        }
        self.resolve_local(keyword, id);
    }

    fn visit_list(&mut self, elements: &[Expr], _id: usize) {
        for element in elements {
            self.resolve_expr(element);
        }
    }

    fn visit_dict(&mut self, entries: &[(Expr, Expr)], _id: usize) {
        for (key, value) in entries {
            self.resolve_expr(key);
            self.resolve_expr(value);
        }
    }

    fn visit_subscript(&mut self, object: &Expr, index: &Expr, _paren: &Token, _id: usize) {
        self.resolve_expr(object);
        self.resolve_expr(index);
    }

    fn visit_subscript_assign(
        &mut self,
        object: &Expr,
        index: &Expr,
        value: &Expr,
        _paren: &Token,
        _id: usize,
    ) {
        self.resolve_expr(object);
        self.resolve_expr(index);
        self.resolve_expr(value);
    }

    fn visit_interpolate(&mut self, parts: &[Expr], _id: usize) {
        for part in parts {
            self.resolve_expr(part);
        }
    }

    fn visit_lambda(&mut self, params: &[Parameter], body: &[Stmt], _id: usize) {
        self.resolve_function(params, body, FunctionType::Function);
    }
}
