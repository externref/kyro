use super::environment::Environment;
use crate::interpreter::class::KyroClass;
use crate::interpreter::instance::KyroInstance;
use crate::primitives;
use crate::{
    interpreter::{
        callable::KyroCallable, environment::EnvRef, function::KyroFunction,
        runtime_error::RuntimeError, value::Value,
    },
    parser::{
        expr::{Expr, ExprVisitor},
        stmt::Stmt,
        tokens::{Literal, Token, TokenType},
    },
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Interpreter {
    pub environment: EnvRef,
    pub locals: HashMap<usize, usize>,
    pub next_id: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();

        env.define(
            "use".to_string(),
            Value::Callable(Rc::new(crate::stdlib::Use)),
        );

        Self {
            environment: Rc::new(RefCell::new(env)),
            locals: HashMap::new(),
            next_id: 0,
        }
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
                println!("{}", value);
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
                    self.execute(body)?;
                }
            }
            Stmt::Function { .. } => {
                let function = KyroFunction::new(stmt.clone(), self.environment.clone(), false);

                let name = match stmt {
                    Stmt::Function { name, .. } => name.lexeme.clone(),
                    _ => unreachable!(),
                };

                self.environment
                    .borrow_mut()
                    .define(name, Value::Callable(Rc::new(function)));
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
            } => {
                let superclass_val = if let Some(super_expr) = super_class {
                    let val = self.interpret(super_expr)?;
                    if let Value::Class(cls) = val {
                        Some(cls)
                    } else {
                        return Err(RuntimeError::new(
                            match super_expr {
                                Expr::Variable { name, .. } => name.clone(),
                                _ => name.clone(),
                            },
                            "Superclass must be a class.",
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
                    if let Stmt::Function { name: mname, .. } = method {
                        let is_initializer = mname.lexeme == "init";
                        let function = KyroFunction::new(
                            method.clone(),
                            self.environment.clone(),
                            is_initializer,
                        );

                        method_map.insert(mname.lexeme.clone(), function);
                    }
                }

                let class = KyroClass {
                    name: name.lexeme.clone(),
                    superclass: superclass_val.clone(),
                    methods: method_map,
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
            } => {
                match self.execute(try_branch) {
                    Ok(_) => {}

                    Err(RuntimeError::Return(v)) => {
                        return Err(RuntimeError::Return(v));
                    }

                    Err(RuntimeError::Error { token: _, value }) => {
                        // Catches the raw Value directly [13.1]
                        let catch_env = Rc::new(RefCell::new(Environment::from_enclosing(
                            self.environment.clone(),
                        )));
                        catch_env
                            .borrow_mut()
                            .define(exception_var.lexeme.clone(), value); // Binds raw Value [13.1]

                        let previous_env = std::mem::replace(&mut self.environment, catch_env);
                        let result = self.execute(catch_branch);
                        self.environment = previous_env;
                        result?;
                    }
                }
            }
            Stmt::Throw { keyword, value } => {
                let err_val = self.interpret(value)?;
                return Err(RuntimeError::Error {
                    token: keyword.clone(),
                    value: err_val, // Returns the original Value without stringifying! [12.1]
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
                    RuntimeError::new(
                        name.clone(),
                        format!("Undefined variable '{}'.", name.lexeme),
                    )
                },
            )
        } else {
            self.environment.borrow().get(&name.lexeme).ok_or_else(|| {
                RuntimeError::new(
                    name.clone(),
                    format!("Undefined variable '{}'.", name.lexeme),
                )
            })
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
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operand must be a number.",
                )),
            },
            TokenType::Bang => Ok(Value::Bool(!is_truthy(&right))),
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
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be two numbers or two strings.",
                )),
            },
            TokenType::Minus => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be numbers.",
                )),
            },
            TokenType::Star => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be numbers.",
                )),
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be numbers.",
                )),
            },
            TokenType::Greater => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be numbers.",
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be numbers.",
                )),
            },
            TokenType::Less => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be numbers.",
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be numbers.",
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
                Err(RuntimeError::new(
                    name.clone(),
                    format!("Undefined variable '{}'.", name.lexeme),
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
        arguments: &[Expr],
    ) -> Result<Value, RuntimeError> {
        let callee = callee.accept(self)?;

        let mut args = Vec::new();
        for arg in arguments {
            args.push(arg.accept(self)?);
        }

        match callee {
            Value::Callable(func) => {
                if args.len() != func.arity() {
                    return Err(RuntimeError::new(
                        paren.clone(),
                        format!(
                            "Expected {} arguments but got {}.",
                            func.arity(),
                            args.len()
                        ),
                    ));
                }

                func.call(self, args)
            }

            Value::Class(class) => {
                let instance = Rc::new(RefCell::new(KyroInstance {
                    class: class.clone(),
                    fields: std::collections::HashMap::new(),
                }));

                let initializer = class.find_method("init");
                let arity = initializer.as_ref().map(|i| i.arity()).unwrap_or(0);

                if args.len() != arity {
                    return Err(RuntimeError::new(
                        paren.clone(),
                        format!("Expected {} arguments but got {}.", arity, args.len()),
                    ));
                }

                if let Some(init) = initializer {
                    let bound = init.bind(instance.clone());
                    bound.call(self, args)?;
                }

                Ok(Value::Instance(instance))
            }

            _ => Err(RuntimeError::new(
                paren.clone(),
                "Can only call functions and classes.",
            )),
        }
    }

    fn visit_get(&mut self, object: &Expr, name: &Token) -> Result<Value, RuntimeError> {
        let obj = object.accept(self)?;

        match obj {
            Value::Instance(instance) => {
                if let Some(value) = instance.borrow().fields.get(&name.lexeme) {
                    return Ok(value.clone());
                }

                if let Some(method) = instance.borrow().class.find_method(&name.lexeme) {
                    let bound_method = method.bind(instance.clone());
                    return Ok(Value::Callable(Rc::new(bound_method)));
                }

                Err(RuntimeError::new(
                    name.clone(),
                    format!("Undefined property '{}'.", name.lexeme),
                ))
            }
            Value::List(list) => primitives::get_list_method(list.clone(), name),
            Value::Dict(dict) => primitives::get_dict_method(dict.clone(), name),
            Value::String(s) => primitives::get_string_method(s.clone(), name),
            Value::Number(n) => primitives::get_number_method(n, name),
            _ => Err(RuntimeError::new(
                name.clone(),
                "Only instances, lists, dictionaries, strings, and numbers have properties.",
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

            _ => Err(RuntimeError::new(
                name.clone(),
                "Only instances have fields.",
            )),
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
            RuntimeError::new(
                keyword.clone(),
                "Internal error: unresolved superclass expression.",
            )
        })?;

        let superclass_val = Environment::get_at(self.environment.clone(), *distance, "super")
            .ok_or_else(|| {
                RuntimeError::new(
                    keyword.clone(),
                    "Internal error: superclass not found in environment.",
                )
            })?;

        let superclass = match superclass_val {
            Value::Class(cls) => cls,
            _ => unreachable!(),
        };

        let instance_val = Environment::get_at(self.environment.clone(), *distance - 1, "this")
            .ok_or_else(|| {
                RuntimeError::new(
                    keyword.clone(),
                    "Internal error: instance 'this' not found in environment.",
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
            Err(RuntimeError::new(
                method.clone(),
                format!("Undefined property '{}'.", method.lexeme),
            ))
        }
    }

    fn visit_list(&mut self, elements: &[Expr], _id: usize) -> Result<Value, RuntimeError> {
        let mut vals = Vec::new();
        for elem in elements {
            vals.push(elem.accept(self)?);
        }
        Ok(Value::List(Rc::new(RefCell::new(vals))))
    }

    fn visit_dict(&mut self, entries: &[(Expr, Expr)], _id: usize) -> Result<Value, RuntimeError> {
        let mut map = HashMap::new();
        for (key_expr, val_expr) in entries {
            let key_val = key_expr.accept(self)?;
            let val_val = val_expr.accept(self)?;
            let key_str = match &key_val {
                Value::String(s) => s.clone(),
                _ => key_val.to_string(),
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
                        return Err(RuntimeError::new(
                            paren.clone(),
                            "List index must be a number.",
                        ));
                    }
                };

                let i = num as usize;
                if num < 0.0 || i >= borrowed.len() {
                    return Err(RuntimeError::new(
                        paren.clone(),
                        "List index out of bounds.",
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
            _ => Err(RuntimeError::new(
                paren.clone(),
                "Only lists and dictionaries support subscript indexing.",
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
                        return Err(RuntimeError::new(
                            paren.clone(),
                            "List index must be a number.",
                        ));
                    }
                };

                let i = num as usize;
                if num < 0.0 || i >= borrowed.len() {
                    return Err(RuntimeError::new(
                        paren.clone(),
                        "List index out of bounds.",
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
            _ => Err(RuntimeError::new(
                paren.clone(),
                "Only lists and dictionaries support subscript assignment.",
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
}
