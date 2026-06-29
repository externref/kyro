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
    interpreter::{
        callable::KyroCallable, class::KyroClass, environment::EnvRef, environment::Environment,
        function::KyroFunction, instance::KyroInstance, runtime_error::RuntimeError, value::Value,
    },
    parser::{
        expr::{Argument, Expr, ExprVisitor},
        stmt::{Parameter, Stmt},
        tokens::{Literal, Token, TokenType},
    },
    primitives,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Interpreter {
    pub environment: EnvRef,
    pub locals: HashMap<usize, usize>,
    pub next_id: usize,
    pub modules: HashMap<String, Value>,
}

static VERSION: &str = include_str!("../.version");

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();

        env.define(
            "__version__".to_string(),
            Value::String(VERSION.to_string()),
        );
        env.define(
            "use".to_string(),
            Value::Callable(Rc::new(crate::stdlib::Use)),
        );
        env.define(
            "__name__".to_string(),
            Value::String("__main__".to_string()),
        );
        env.define(
            "id".to_string(),
            Value::Callable(Rc::new(crate::stdlib::IdFn)),
        );
        env.define(
            "dir".to_string(),
            Value::Callable(Rc::new(crate::stdlib::DirFn)),
        );
        env.define(
            "is_instance".to_string(),
            Value::Callable(Rc::new(crate::stdlib::IsInstanceFn)),
        );

        Self {
            environment: Rc::new(RefCell::new(env)),
            locals: HashMap::new(),
            next_id: 0,
            modules: HashMap::new(),
        }
    }

    pub fn stringify(&mut self, value: &Value) -> String {
        match value {
            Value::Instance(instance) => {
                if let Some(method) = instance.borrow().class.find_method("__str__") {
                    let bound = method.bind(instance.clone());
                    if let Ok(res) = bound.call(self, Vec::new()) {
                        return res.to_string();
                    }
                }
                if let Some(method) = instance.borrow().class.find_method("to_string") {
                    let bound = method.bind(instance.clone());
                    if let Ok(res) = bound.call(self, Vec::new()) {
                        return res.to_string();
                    }
                }
                format!("<instance {}>", instance.borrow().class.name)
            }
            Value::Class(class) => {
                format!("<class {}>", class.name)
            }
            _ => value.to_string(),
        }
    }

    pub fn raise_error(&self, class_name: &str, message: &str, token: Token) -> RuntimeError {
        let err_class = self.environment.borrow().get(class_name);
        let value = if let Some(Value::Class(class)) = err_class {
            let instance = Rc::new(RefCell::new(KyroInstance {
                class: class.clone(),
                fields: {
                    let mut f = HashMap::new();
                    f.insert("message".to_string(), Value::String(message.to_string()));
                    f
                },
            }));
            Value::Instance(instance)
        } else {
            Value::String(format!("{class_name}: {message}"))
        };

        RuntimeError::Error { token, value }
    }

    pub fn resolve(&mut self, id: usize, depth: usize) {
        self.locals.insert(id, depth);
    }

    pub fn interpret(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.interpret(expr)?;
            }
            Stmt::Echo(expr) => {
                let value = self.interpret(expr)?;
                let str_val = self.stringify(&value);
                println!("{}", str_val);
            }

            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.interpret(expr)?,
                    None => Value::Nil,
                };

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);
            }

            Stmt::VarDestructure {
                names,
                is_dict,
                initializer,
            } => {
                let init_val = self.interpret(initializer)?;
                if *is_dict {
                    let dict = match init_val {
                        Value::Dict(d) => d,
                        _ => {
                            return Err(self.raise_error(
                                "TypeError",
                                "Right-hand side of dictionary destructuring must be a dictionary.",
                                Token::new(
                                    TokenType::Identifier,
                                    "destructure".to_string(),
                                    None,
                                    0,
                                ),
                            ));
                        }
                    };
                    for name in names {
                        let val = dict
                            .borrow()
                            .get(&name.lexeme)
                            .cloned()
                            .unwrap_or(Value::Nil);
                        self.environment
                            .borrow_mut()
                            .define(name.lexeme.clone(), val);
                    }
                } else {
                    let list = match init_val {
                        Value::List(l) => l,
                        _ => {
                            return Err(self.raise_error(
                                "TypeError",
                                "Right-hand side of list destructuring must be a list.",
                                Token::new(
                                    TokenType::Identifier,
                                    "destructure".to_string(),
                                    None,
                                    0,
                                ),
                            ));
                        }
                    };
                    for (i, name) in names.iter().enumerate() {
                        let val = list.borrow().get(i).cloned().unwrap_or(Value::Nil);
                        self.environment
                            .borrow_mut()
                            .define(name.lexeme.clone(), val);
                    }
                }
            }

            Stmt::Block(statements) => {
                let environment = Rc::new(RefCell::new(Environment::from_enclosing(
                    self.environment.clone(),
                )));

                self.execute_block(statements, environment)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let value = self.interpret(condition)?;

                if is_truthy(&value) {
                    self.execute(then_branch)?;
                } else if let Some(branch) = else_branch {
                    self.execute(branch)?;
                }
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.interpret(condition)?) {
                    match self.execute(body) {
                        Ok(_) => {}
                        Err(RuntimeError::Break) => break,
                        Err(RuntimeError::Continue) => continue,
                        Err(e) => return Err(e),
                    }
                }
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                let previous_env = self.environment.clone();
                let loop_env = Rc::new(RefCell::new(Environment::from_enclosing(
                    self.environment.clone(),
                )));
                self.environment = loop_env;

                let mut run_loop = || -> Result<(), RuntimeError> {
                    if let Some(init) = initializer {
                        self.execute(init)?;
                    }
                    while is_truthy(&self.interpret(condition)?) {
                        match self.execute(body) {
                            Ok(_) => {}
                            Err(RuntimeError::Break) => break,
                            Err(RuntimeError::Continue) => {}
                            Err(e) => return Err(e),
                        }
                        if let Some(inc) = increment {
                            self.interpret(inc)?;
                        }
                    }
                    Ok(())
                };

                let result = run_loop();
                self.environment = previous_env;
                result?;
            }
            Stmt::ForIn {
                var_name,
                collection,
                body,
            } => {
                let col_val = self.interpret(collection)?;
                match col_val {
                    Value::List(list) => {
                        let elements = list.borrow().clone();
                        for elem in elements {
                            let loop_env = Rc::new(RefCell::new(Environment::from_enclosing(
                                self.environment.clone(),
                            )));
                            loop_env.borrow_mut().define(var_name.lexeme.clone(), elem);
                            let previous_env = std::mem::replace(&mut self.environment, loop_env);

                            let result = self.execute(body);
                            self.environment = previous_env;

                            match result {
                                Ok(_) => {}
                                Err(RuntimeError::Break) => break,
                                Err(RuntimeError::Continue) => {}
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Value::Dict(dict) => {
                        let keys: Vec<String> = dict.borrow().keys().cloned().collect();
                        for key in keys {
                            let loop_env = Rc::new(RefCell::new(Environment::from_enclosing(
                                self.environment.clone(),
                            )));
                            loop_env
                                .borrow_mut()
                                .define(var_name.lexeme.clone(), Value::String(key));
                            let previous_env = std::mem::replace(&mut self.environment, loop_env);

                            let result = self.execute(body);
                            self.environment = previous_env;

                            match result {
                                Ok(_) => {}
                                Err(RuntimeError::Break) => break,
                                Err(RuntimeError::Continue) => {}
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Value::Instance(instance) => {
                        let next_method = instance.borrow().class.find_method("__next__");

                        if let Some(next_method) = next_method {
                            loop {
                                let bound = next_method.bind(instance.clone());
                                let elem = bound.call(self, Vec::new())?;
                                if let Value::Nil = elem {
                                    break;
                                }

                                let loop_env = Rc::new(RefCell::new(Environment::from_enclosing(
                                    self.environment.clone(),
                                )));
                                loop_env.borrow_mut().define(var_name.lexeme.clone(), elem);
                                let previous_env =
                                    std::mem::replace(&mut self.environment, loop_env);

                                let result = self.execute(body);
                                self.environment = previous_env;

                                match result {
                                    Ok(_) => {}
                                    Err(RuntimeError::Break) => break,
                                    Err(RuntimeError::Continue) => {}
                                    Err(e) => return Err(e),
                                }
                            }
                        } else {
                            return Err(self.raise_error(
                                "TypeError",
                                "Only lists, dictionaries, and custom iterators are iterable.",
                                var_name.clone(),
                            ));
                        }
                    }
                    _ => {
                        return Err(self.raise_error(
                            "TypeError",
                            "Only lists, dictionaries, and custom iterators are iterable.",
                            var_name.clone(),
                        ));
                    }
                }
            }
            Stmt::Break { keyword: _ } => {
                return Err(RuntimeError::Break);
            }
            Stmt::Continue { keyword: _ } => {
                return Err(RuntimeError::Continue);
            }
            Stmt::Function { name, doc, .. } => {
                let function =
                    KyroFunction::new(stmt.clone(), self.environment.clone(), false, doc.clone());

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Value::Callable(Rc::new(function)));
            }
            Stmt::Return { value, .. } => {
                let val = if let Some(expr) = value {
                    self.interpret(expr)?
                } else {
                    Value::Nil
                };

                return Err(RuntimeError::Return(val));
            }
            Stmt::Class {
                name,
                super_class,
                methods,
                doc,
            } => {
                let superclass_val = if let Some(super_expr) = super_class {
                    let val = self.interpret(super_expr)?;
                    if let Value::Class(cls) = val {
                        Some(cls)
                    } else {
                        return Err(self.raise_error(
                            "TypeError",
                            "Superclass must be a class.",
                            match super_expr {
                                Expr::Variable { name, .. } => name.clone(),
                                _ => name.clone(),
                            },
                        ));
                    }
                } else {
                    None
                };

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Value::Nil);

                let previous_env = self.environment.clone();
                if let Some(ref supercls) = superclass_val {
                    let mut super_env = Environment::from_enclosing(self.environment.clone());
                    super_env.define("super".to_string(), Value::Class(supercls.clone()));
                    self.environment = Rc::new(RefCell::new(super_env));
                }

                let mut method_map = std::collections::HashMap::new();

                for method in methods {
                    if let Stmt::Function {
                        name: mname,
                        doc: mdoc,
                        ..
                    } = method
                    {
                        let is_initializer = mname.lexeme == "__init__";
                        let function = KyroFunction::new(
                            method.clone(),
                            self.environment.clone(),
                            is_initializer,
                            mdoc.clone(),
                        );

                        method_map.insert(mname.lexeme.clone(), function);
                    }
                }

                let class = KyroClass {
                    name: name.lexeme.clone(),
                    superclass: superclass_val.clone(),
                    methods: method_map,
                    doc: doc.clone(),
                };

                let class_val = Value::Class(Rc::new(class));

                if super_class.is_some() {
                    self.environment = previous_env;
                }

                self.environment
                    .borrow_mut()
                    .assign(&name.lexeme, class_val);
            }
            Stmt::TryCatch {
                try_branch,
                exception_var,
                catch_branch,
            } => match self.execute(try_branch) {
                Ok(_) => {}

                Err(RuntimeError::Return(v)) => {
                    return Err(RuntimeError::Return(v));
                }

                Err(RuntimeError::Error { token: _, value }) => {
                    let catch_env = Rc::new(RefCell::new(Environment::from_enclosing(
                        self.environment.clone(),
                    )));
                    catch_env
                        .borrow_mut()
                        .define(exception_var.lexeme.clone(), value);

                    let previous_env = std::mem::replace(&mut self.environment, catch_env);
                    let result = self.execute(catch_branch);
                    self.environment = previous_env;
                    result?;
                }
                Err(RuntimeError::Break) => return Err(RuntimeError::Break),
                Err(RuntimeError::Continue) => return Err(RuntimeError::Continue),
            },
            Stmt::Throw { keyword, value } => {
                let err_val = self.interpret(value)?;
                return Err(RuntimeError::Error {
                    token: keyword.clone(),
                    value: err_val,
                });
            }
        }
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: EnvRef,
    ) -> Result<(), RuntimeError> {
        let previous = std::mem::replace(&mut self.environment, environment);

        for stmt in statements {
            match self.execute(stmt) {
                Ok(_) => {}

                Err(RuntimeError::Return(v)) => {
                    self.environment = previous;
                    return Err(RuntimeError::Return(v));
                }

                Err(e) => {
                    self.environment = previous;
                    return Err(e);
                }
            }
        }

        self.environment = previous;

        Ok(())
    }

    fn look_up_variable(&self, name: &Token, id: usize) -> Result<Value, RuntimeError> {
        if let Some(distance) = self.locals.get(&id) {
            Environment::get_at(self.environment.clone(), *distance, &name.lexeme).ok_or_else(
                || {
                    self.raise_error(
                        "ValueError",
                        &format!("Undefined variable '{}'.", name.lexeme),
                        name.clone(),
                    )
                },
            )
        } else {
            self.environment.borrow().get(&name.lexeme).ok_or_else(|| {
                self.raise_error(
                    "ValueError",
                    &format!("Undefined variable '{}'.", name.lexeme),
                    name.clone(),
                )
            })
        }
    }

    fn resolve_callable_arguments(
        &mut self,
        func: &Rc<dyn KyroCallable>,
        paren: &Token,
        arguments: &[Argument],
    ) -> Result<Vec<Value>, RuntimeError> {
        let param_names = func.parameter_names();

        if param_names.is_empty() {
            let mut args = Vec::new();
            for arg in arguments {
                match arg {
                    Argument::Positional(expr) => {
                        args.push(expr.accept(self)?);
                    }
                    Argument::Keyword { name, .. } => {
                        return Err(self.raise_error(
                            "TypeError",
                            &format!(
                                "Keyword arguments are not supported by function '{}'.",
                                func.name()
                            ),
                            name.clone(),
                        ));
                    }
                }
            }

            if args.len() != func.arity() {
                return Err(self.raise_error(
                    "TypeError",
                    &format!(
                        "Expected {} arguments but got {}.",
                        func.arity(),
                        args.len()
                    ),
                    paren.clone(),
                ));
            }
            Ok(args)
        } else {
            let mut resolved_args = vec![None; param_names.len()];
            let mut positional_count = 0;

            for arg in arguments {
                match arg {
                    Argument::Positional(expr) => {
                        if positional_count >= param_names.len() {
                            return Err(self.raise_error(
                                "TypeError",
                                &format!(
                                    "Too many positional arguments passed to function '{}'.",
                                    func.name()
                                ),
                                paren.clone(),
                            ));
                        }
                        let val = expr.accept(self)?;
                        resolved_args[positional_count] = Some(val);
                        positional_count += 1;
                    }
                    Argument::Keyword { name, value } => {
                        let param_idx = param_names.iter().position(|p| p == &name.lexeme);
                        match param_idx {
                            Some(idx) => {
                                if resolved_args[idx].is_some() {
                                    return Err(self.raise_error(
                                        "TypeError",
                                        &format!(
                                            "Duplicate value provided for argument '{}'.",
                                            name.lexeme
                                        ),
                                        name.clone(),
                                    ));
                                }
                                let val = value.accept(self)?;
                                resolved_args[idx] = Some(val);
                            }
                            None => {
                                return Err(self.raise_error(
                                    "TypeError",
                                    &format!(
                                        "Unknown keyword argument '{}' on function '{}'.",
                                        name.lexeme,
                                        func.name()
                                    ),
                                    name.clone(),
                                ));
                            }
                        }
                    }
                }
            }

            for (i, param_name) in param_names.iter().enumerate() {
                if resolved_args[i].is_none() {
                    if let Some(def_val_res) = func.default_value(self, param_name) {
                        resolved_args[i] = Some(def_val_res?);
                    } else {
                        return Err(self.raise_error(
                            "TypeError",
                            &format!(
                                "Missing required argument '{}' for function '{}'.",
                                param_name,
                                func.name()
                            ),
                            paren.clone(),
                        ));
                    }
                }
            }

            Ok(resolved_args.into_iter().map(|o| o.unwrap()).collect())
        }
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Bool(b) => *b,
        _ => true,
    }
}

fn is_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Nil, Value::Nil) => true,
        (Value::Nil, _) => false,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        _ => false,
    }
}

impl ExprVisitor<Result<Value, RuntimeError>> for Interpreter {
    fn visit_literal(&mut self, literal: &Literal) -> Result<Value, RuntimeError> {
        Ok(match literal {
            Literal::Number(n) => Value::Number(*n),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::Nil => Value::Nil,
        })
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
        let right = right.accept(self)?;
        match operator.r#type {
            TokenType::Minus => match right {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operand must be a number.",
                    operator.clone(),
                )),
            },
            TokenType::Bang => Ok(Value::Bool(!is_truthy(&right))),
            TokenType::Tilde => match right {
                Value::Number(n) => Ok(Value::Number((!(n as i64)) as f64)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operand must be a number.",
                    operator.clone(),
                )),
            },
            _ => unreachable!(),
        }
    }

    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        let left = left.accept(self)?;
        let right = right.accept(self)?;
        match operator.r#type {
            TokenType::Plus => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be two numbers or two strings.",
                    operator.clone(),
                )),
            },
            TokenType::Minus => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::Star => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::Ampersand => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    Ok(Value::Number(((a as i64) & (b as i64)) as f64))
                }
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::Pipe => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    Ok(Value::Number(((a as i64) | (b as i64)) as f64))
                }
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::Caret => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    Ok(Value::Number(((a as i64) ^ (b as i64)) as f64))
                }
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::LessLess => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    Ok(Value::Number(((a as i64) << (b as i64)) as f64))
                }
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::GreaterGreater => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    Ok(Value::Number(((a as i64) >> (b as i64)) as f64))
                }
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::Greater => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::Less => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                _ => Err(self.raise_error(
                    "TypeError",
                    "Operands must be numbers.",
                    operator.clone(),
                )),
            },
            TokenType::EqualEqual => Ok(Value::Bool(is_equal(&left, &right))),
            TokenType::BangEqual => Ok(Value::Bool(!is_equal(&left, &right))),
            _ => unreachable!(),
        }
    }

    fn visit_variable(&mut self, name: &Token, id: usize) -> Result<Value, RuntimeError> {
        self.look_up_variable(name, id)
    }

    fn visit_assign(
        &mut self,
        name: &Token,
        value_expr: &Expr,
        id: usize,
    ) -> Result<Value, RuntimeError> {
        let value = value_expr.accept(self)?;

        if let Some(distance) = self.locals.get(&id) {
            Environment::assign_at(
                self.environment.clone(),
                *distance,
                &name.lexeme,
                value.clone(),
            );
            Ok(value)
        } else {
            if self
                .environment
                .borrow_mut()
                .assign(&name.lexeme, value.clone())
            {
                Ok(value)
            } else {
                Err(self.raise_error(
                    "ValueError",
                    &format!("Undefined variable '{}'.", name.lexeme),
                    name.clone(),
                ))
            }
        }
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        let left = left.accept(self)?;

        match operator.r#type {
            TokenType::Or => {
                if is_truthy(&left) {
                    return Ok(left);
                }
            }
            TokenType::And => {
                if !is_truthy(&left) {
                    return Ok(left);
                }
            }
            _ => unreachable!(),
        }

        right.accept(self)
    }

    fn visit_call(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &[Argument],
    ) -> Result<Value, RuntimeError> {
        let callee = callee.accept(self)?;

        match callee {
            Value::Callable(func) => {
                let args = self.resolve_callable_arguments(&func, paren, arguments)?;
                func.call(self, args)
            }

            Value::Class(class) => {
                let instance = Rc::new(RefCell::new(KyroInstance {
                    class: class.clone(),
                    fields: std::collections::HashMap::new(),
                }));

                let initializer = class.find_method("__init__");
                let args = if let Some(init) = &initializer {
                    let init_callable: Rc<dyn KyroCallable> = Rc::new(init.clone());
                    self.resolve_callable_arguments(&init_callable, paren, arguments)?
                } else {
                    if !arguments.is_empty() {
                        return Err(self.raise_error(
                            "TypeError",
                            "Expected 0 arguments but got some.",
                            paren.clone(),
                        ));
                    }
                    Vec::new()
                };

                if let Some(init) = initializer {
                    let bound = init.bind(instance.clone());
                    bound.call(self, args)?;
                }

                Ok(Value::Instance(instance))
            }

            _ => Err(self.raise_error(
                "TypeError",
                "Can only call functions and classes.",
                paren.clone(),
            )),
        }
    }

    fn visit_get(&mut self, object: &Expr, name: &Token) -> Result<Value, RuntimeError> {
        let obj = object.accept(self)?;

        match obj {
            Value::Instance(instance) => {
                if name.lexeme == "__class__" {
                    return Ok(Value::Class(instance.borrow().class.clone()));
                }

                if name.lexeme == "__doc__" {
                    let class_borrow = instance.borrow();
                    let doc_val = class_borrow
                        .class
                        .doc
                        .as_ref()
                        .map_or(Value::Nil, |doc_str| Value::String(doc_str.clone()));
                    return Ok(doc_val);
                }

                if let Some(value) = instance.borrow().fields.get(&name.lexeme) {
                    return Ok(value.clone());
                }

                if let Some(method) = instance.borrow().class.find_method(&name.lexeme) {
                    let bound_method = method.bind(instance.clone());
                    return Ok(Value::Callable(Rc::new(bound_method)));
                }

                Err(self.raise_error(
                    "AttributeError",
                    &format!("Undefined property '{}'.", name.lexeme),
                    name.clone(),
                ))
            }
            Value::List(list) => primitives::get_list_method(list.clone(), name),
            Value::Dict(dict) => primitives::get_dict_method(dict.clone(), name),
            Value::String(s) => primitives::get_string_method(s.clone(), name),
            Value::Number(n) => primitives::get_number_method(n, name),
            Value::Callable(callable) => {
                if name.lexeme == "__name__" {
                    Ok(Value::String(callable.name().to_string()))
                } else if name.lexeme == "__doc__" {
                    let doc_val = callable
                        .doc()
                        .map_or(Value::Nil, |doc_str| Value::String(doc_str.to_string()));
                    Ok(doc_val)
                } else {
                    Err(self.raise_error(
                        "AttributeError",
                        &format!("Undefined property '{}' on callable.", name.lexeme),
                        name.clone(),
                    ))
                }
            }
            Value::Class(class) => {
                if name.lexeme == "__name__" {
                    Ok(Value::String(class.name.clone()))
                } else if name.lexeme == "__doc__" {
                    let doc_val = class
                        .doc
                        .as_ref()
                        .map_or(Value::Nil, |doc_str| Value::String(doc_str.clone()));
                    Ok(doc_val)
                } else {
                    Err(self.raise_error(
                        "AttributeError",
                        &format!("Undefined property '{}' on class.", name.lexeme),
                        name.clone(),
                    ))
                }
            }
            _ => Err(self.raise_error(
                "TypeError",
                "Only instances, lists, dictionaries, strings, and numbers have properties.",
                name.clone(),
            )),
        }
    }

    fn visit_set(
        &mut self,
        object: &Expr,
        name: &Token,
        value: &Expr,
    ) -> Result<Value, RuntimeError> {
        let obj = object.accept(self)?;
        let val = value.accept(self)?;

        match obj {
            Value::Instance(instance) => {
                instance
                    .borrow_mut()
                    .fields
                    .insert(name.lexeme.clone(), val.clone());

                Ok(val)
            }

            _ => Err(self.raise_error("TypeError", "Only instances have fields.", name.clone())),
        }
    }

    fn visit_this(&mut self, keyword: &Token, id: usize) -> Result<Value, RuntimeError> {
        self.look_up_variable(keyword, id)
    }

    fn visit_super(
        &mut self,
        keyword: &Token,
        method: &Token,
        id: usize,
    ) -> Result<Value, RuntimeError> {
        let distance = self.locals.get(&id).ok_or_else(|| {
            self.raise_error(
                "ValueError",
                "Internal error: unresolved superclass expression.",
                keyword.clone(),
            )
        })?;

        let superclass_val = Environment::get_at(self.environment.clone(), *distance, "super")
            .ok_or_else(|| {
                self.raise_error(
                    "ValueError",
                    "Internal error: superclass not found in environment.",
                    keyword.clone(),
                )
            })?;

        let superclass = match superclass_val {
            Value::Class(cls) => cls,
            _ => unreachable!(),
        };

        let instance_val = Environment::get_at(self.environment.clone(), *distance - 1, "this")
            .ok_or_else(|| {
                self.raise_error(
                    "ValueError",
                    "Internal error: instance 'this' not found in environment.",
                    keyword.clone(),
                )
            })?;

        let instance = match instance_val {
            Value::Instance(inst) => inst,
            _ => unreachable!(),
        };

        if let Some(superclass_method) = superclass.find_method(&method.lexeme) {
            let bound_method = superclass_method.bind(instance.clone());
            Ok(Value::Callable(Rc::new(bound_method)))
        } else {
            Err(self.raise_error(
                "AttributeError",
                &format!("Undefined property '{}'.", method.lexeme),
                method.clone(),
            ))
        }
    }

    fn visit_list(&mut self, elements: &[Expr], _id: usize) -> Result<Value, RuntimeError> {
        let vals: Result<Vec<Value>, RuntimeError> =
            elements.iter().map(|elem| elem.accept(self)).collect();
        Ok(Value::List(Rc::new(RefCell::new(vals?))))
    }

    fn visit_dict(&mut self, entries: &[(Expr, Expr)], _id: usize) -> Result<Value, RuntimeError> {
        let mut map = HashMap::new();
        for (key_expr, val_expr) in entries {
            let key_val = key_expr.accept(self)?;
            let val_val = val_expr.accept(self)?;
            let key_str = match key_val {
                Value::String(s) => s,
                other => other.to_string(),
            };
            map.insert(key_str, val_val);
        }
        Ok(Value::Dict(Rc::new(RefCell::new(map))))
    }

    fn visit_subscript(
        &mut self,
        object: &Expr,
        index: &Expr,
        paren: &Token,
        _id: usize,
    ) -> Result<Value, RuntimeError> {
        let obj = object.accept(self)?;
        let idx = index.accept(self)?;

        match obj {
            Value::List(list) => {
                let borrowed = list.borrow();
                let num = match idx {
                    Value::Number(n) => n,
                    _ => {
                        return Err(self.raise_error(
                            "TypeError",
                            "List index must be a number.",
                            paren.clone(),
                        ));
                    }
                };

                let i = num as usize;
                if num < 0.0 || i >= borrowed.len() {
                    return Err(self.raise_error(
                        "IndexError",
                        "List index out of bounds.",
                        paren.clone(),
                    ));
                }
                Ok(borrowed[i].clone())
            }
            Value::Dict(dict) => {
                let borrowed = dict.borrow();
                let key_str = match idx {
                    Value::String(s) => s,
                    _ => idx.to_string(),
                };
                if let Some(val) = borrowed.get(&key_str) {
                    Ok(val.clone())
                } else {
                    Ok(Value::Nil)
                }
            }
            _ => Err(self.raise_error(
                "TypeError",
                "Only lists and dictionaries support subscript indexing.",
                paren.clone(),
            )),
        }
    }

    fn visit_subscript_assign(
        &mut self,
        object: &Expr,
        index: &Expr,
        value: &Expr,
        paren: &Token,
        _id: usize,
    ) -> Result<Value, RuntimeError> {
        let obj = object.accept(self)?;
        let idx = index.accept(self)?;
        let val = value.accept(self)?;

        match obj {
            Value::List(list) => {
                let mut borrowed = list.borrow_mut();
                let num = match idx {
                    Value::Number(n) => n,
                    _ => {
                        return Err(self.raise_error(
                            "TypeError",
                            "List index must be a number.",
                            paren.clone(),
                        ));
                    }
                };

                let i = num as usize;
                if num < 0.0 || i >= borrowed.len() {
                    return Err(self.raise_error(
                        "IndexError",
                        "List index out of bounds.",
                        paren.clone(),
                    ));
                }
                borrowed[i] = val.clone();
                Ok(val)
            }
            Value::Dict(dict) => {
                let mut borrowed = dict.borrow_mut();
                let key_str = match idx {
                    Value::String(s) => s,
                    _ => idx.to_string(),
                };
                borrowed.insert(key_str, val.clone());
                Ok(val)
            }
            _ => Err(self.raise_error(
                "TypeError",
                "Only lists and dictionaries support subscript assignment.",
                paren.clone(),
            )),
        }
    }

    fn visit_interpolate(&mut self, parts: &[Expr], _id: usize) -> Result<Value, RuntimeError> {
        let mut result = String::new();
        for part in parts {
            let val = part.accept(self)?;
            result.push_str(&val.to_string());
        }
        Ok(Value::String(result))
    }

    fn visit_lambda(
        &mut self,
        params: &[Parameter],
        body: &[Stmt],
        _id: usize,
    ) -> Result<Value, RuntimeError> {
        let stmt = Stmt::Function {
            name: Token::new(TokenType::Identifier, "anonymous".to_string(), None, 0),
            params: params.to_vec(),
            body: body.to_vec(),
            doc: None,
        };
        let function = KyroFunction::new(stmt, self.environment.clone(), false, None);
        Ok(Value::Callable(Rc::new(function)))
    }
}
