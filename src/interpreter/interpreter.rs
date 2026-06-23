use super::environment::Environment;
use crate::interpreter::class::KyroClass;
use crate::interpreter::instance::KyroInstance;
use crate::stdlib::time::Clock;
use crate::{
    interpreter::{
        environment::EnvRef, function::KyroFunction, runtime_error::RuntimeError, value::Value,
    },
    parser::{
        expr::{Expr, ExprVisitor},
        stmt::Stmt,
        tokens::{Literal, Token, TokenType},
    },
};
use std::{cell::RefCell, rc::Rc};

pub struct Interpreter {
    pub environment: EnvRef,
}
impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();

        env.define("clock".to_string(), Value::Callable(Rc::new(Clock)));

        Self {
            environment: Rc::new(RefCell::new(env)),
        }
    }
    pub fn interpret(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }
    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.interpret(expr)?;
            }
            Stmt::Print(expr) => {
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
                let function = KyroFunction::new(stmt.clone(), self.environment.clone());

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
            Stmt::Class { name, methods } => {
                let mut method_map = std::collections::HashMap::new();

                for method in methods {
                    if let Stmt::Function { name: mname, .. } = method {
                        let function = KyroFunction::new(method.clone(), self.environment.clone());

                        method_map.insert(mname.lexeme.clone(), function);
                    }
                }

                let class = KyroClass {
                    name: name.lexeme.clone(),
                    methods: method_map,
                };

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Value::Class(Rc::new(class)));
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
                _ => Err(RuntimeError::Error {
                    token: operator.clone(),
                    message: "Operand must be a number.".to_string(),
                }),
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

                _ => Err(RuntimeError::Error {
                    token: operator.clone(),
                    message: "Operands must be two numbers or two strings.".to_string(),
                }),
            },
            TokenType::Minus => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),

                _ => Err(RuntimeError::Error {
                    token: operator.clone(),
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::Star => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),

                _ => Err(RuntimeError::Error {
                    token: operator.clone(),
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),

                _ => Err(RuntimeError::Error {
                    token: operator.clone(),
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::Greater => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),

                _ => Err(RuntimeError::Error {
                    token: operator.clone(),
                    message: "Operands must be numbers.".to_string(),
                }),
            },

            TokenType::GreaterEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),

                _ => Err(RuntimeError::Error {
                    token: operator.clone(),
                    message: "Operands must be numbers.".to_string(),
                }),
            },

            TokenType::Less => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),

                _ => Err(RuntimeError::Error {
                    token: operator.clone(),
                    message: "Operands must be numbers.".to_string(),
                }),
            },

            TokenType::LessEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),

                _ => Err(RuntimeError::Error {
                    token: operator.clone(),
                    message: "Operands must be numbers.".to_string(),
                }),
            },

            TokenType::EqualEqual => Ok(Value::Bool(is_equal(&left, &right))),

            TokenType::BangEqual => Ok(Value::Bool(!is_equal(&left, &right))),

            _ => unreachable!(),
        }
    }
    fn visit_variable(&mut self, name: &Token) -> Result<Value, RuntimeError> {
        self.environment
            .borrow()
            .get(&name.lexeme)
            .ok_or(RuntimeError::Error {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme),
            })
    }
    fn visit_assign(&mut self, name: &Token, value_expr: &Expr) -> Result<Value, RuntimeError> {
        let value = value_expr.accept(self)?;

        if self
            .environment
            .borrow_mut()
            .assign(&name.lexeme, value.clone())
        {
            Ok(value)
        } else {
            Err(RuntimeError::Error {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme),
            })
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
                    return Err(RuntimeError::Error {
                        token: paren.clone(),
                        message: format!(
                            "Expected {} arguments but got {}.",
                            func.arity(),
                            args.len()
                        ),
                    });
                }

                func.call(self, args)
            }

            Value::Class(class) => {
                let instance = Rc::new(RefCell::new(KyroInstance {
                    class: class.clone(),
                    fields: std::collections::HashMap::new(),
                }));

                Ok(Value::Instance(instance))
            }

            _ => Err(RuntimeError::Error {
                token: paren.clone(),
                message: "Can only call functions and classes.".to_string(),
            }),
        }
    }
    fn visit_get(&mut self, object: &Expr, name: &Token) -> Result<Value, RuntimeError> {
        let obj = object.accept(self)?;

        match obj {
            Value::Instance(instance) => {
                let inst = instance.borrow();

                if let Some(value) = inst.fields.get(&name.lexeme) {
                    Ok(value.clone())
                } else {
                    Ok(Value::Nil)
                }
            }

            _ => Err(RuntimeError::Error {
                token: name.clone(),
                message: "Only instances have properties.".to_string(),
            }),
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

            _ => Err(RuntimeError::Error {
                token: name.clone(),
                message: "Only instances have fields.".to_string(),
            }),
        }
    }
}
