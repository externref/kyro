use super::environment::Environment;
use crate::interpreter::class::KyroClass;
use crate::interpreter::instance::KyroInstance;
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
                        return Err(RuntimeError::Error {
                            token: match super_expr {
                                Expr::Variable { name, .. } => name.clone(),
                                _ => name.clone(),
                            },
                            message: "Superclass must be a class.".to_string(),
                        });
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
                || RuntimeError::Error {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", name.lexeme),
                },
            )
        } else {
            self.environment
                .borrow()
                .get(&name.lexeme)
                .ok_or_else(|| RuntimeError::Error {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", name.lexeme),
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
                Err(RuntimeError::Error {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", name.lexeme),
                })
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
                let initializer = class.find_method("init");
                let arity = initializer.as_ref().map(|i| i.arity()).unwrap_or(0);
                if args.len() != arity {
                    return Err(RuntimeError::Error {
                        token: paren.clone(),
                        message: format!("Expected {} arguments but got {}.", arity, args.len()),
                    });
                }
                if let Some(init) = initializer {
                    let bound = init.bind(instance.clone());
                    bound.call(self, args)?;
                }
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
                if let Some(value) = instance.borrow().fields.get(&name.lexeme) {
                    return Ok(value.clone());
                }
                if let Some(method) = instance.borrow().class.find_method(&name.lexeme) {
                    let bound_method = method.bind(instance.clone());
                    return Ok(Value::Callable(Rc::new(bound_method)));
                }
                Err(RuntimeError::Error {
                    token: name.clone(),
                    message: format!("Undefined property '{}'.", name.lexeme),
                })
            }
            Value::List(list) => match name.lexeme.as_str() {
                "len" => {
                    struct ListLen {
                        list: Rc<RefCell<Vec<Value>>>,
                    }
                    impl KyroCallable for ListLen {
                        fn arity(&self) -> usize {
                            0
                        }
                        fn call(
                            &self,
                            _: &mut Interpreter,
                            _: Vec<Value>,
                        ) -> Result<Value, RuntimeError> {
                            Ok(Value::Number(self.list.borrow().len() as f64))
                        }
                        fn name(&self) -> &str {
                            "len"
                        }
                    }
                    Ok(Value::Callable(Rc::new(ListLen { list: list.clone() })))
                }
                "push" => {
                    struct ListPush {
                        list: Rc<RefCell<Vec<Value>>>,
                    }
                    impl KyroCallable for ListPush {
                        fn arity(&self) -> usize {
                            1
                        }
                        fn call(
                            &self,
                            _: &mut Interpreter,
                            args: Vec<Value>,
                        ) -> Result<Value, RuntimeError> {
                            self.list.borrow_mut().push(args[0].clone());
                            Ok(Value::Nil)
                        }
                        fn name(&self) -> &str {
                            "push"
                        }
                    }
                    Ok(Value::Callable(Rc::new(ListPush { list: list.clone() })))
                }
                "pop" => {
                    struct ListPop {
                        list: Rc<RefCell<Vec<Value>>>,
                    }
                    impl KyroCallable for ListPop {
                        fn arity(&self) -> usize {
                            0
                        }
                        fn call(
                            &self,
                            _: &mut Interpreter,
                            _: Vec<Value>,
                        ) -> Result<Value, RuntimeError> {
                            let val = self.list.borrow_mut().pop().unwrap_or(Value::Nil);
                            Ok(val)
                        }
                        fn name(&self) -> &str {
                            "pop"
                        }
                    }
                    Ok(Value::Callable(Rc::new(ListPop { list: list.clone() })))
                }
                _ => Err(RuntimeError::Error {
                    token: name.clone(),
                    message: format!("Undefined list method '{}'.", name.lexeme),
                }),
            },
            Value::Dict(dict) => match name.lexeme.as_str() {
                "len" => {
                    struct DictLen {
                        dict: Rc<RefCell<HashMap<String, Value>>>,
                    }
                    impl KyroCallable for DictLen {
                        fn arity(&self) -> usize {
                            0
                        }
                        fn call(
                            &self,
                            _: &mut Interpreter,
                            _: Vec<Value>,
                        ) -> Result<Value, RuntimeError> {
                            Ok(Value::Number(self.dict.borrow().len() as f64))
                        }
                        fn name(&self) -> &str {
                            "len"
                        }
                    }
                    Ok(Value::Callable(Rc::new(DictLen { dict: dict.clone() })))
                }
                "remove" => {
                    struct DictRemove {
                        dict: Rc<RefCell<HashMap<String, Value>>>,
                    }
                    impl KyroCallable for DictRemove {
                        fn arity(&self) -> usize {
                            1
                        }
                        fn call(
                            &self,
                            _: &mut Interpreter,
                            args: Vec<Value>,
                        ) -> Result<Value, RuntimeError> {
                            let key_str = match &args[0] {
                                Value::String(s) => s.clone(),
                                _ => args[0].to_string(),
                            };
                            let removed = self
                                .dict
                                .borrow_mut()
                                .remove(&key_str)
                                .unwrap_or(Value::Nil);
                            Ok(removed)
                        }
                        fn name(&self) -> &str {
                            "remove"
                        }
                    }
                    Ok(Value::Callable(Rc::new(DictRemove { dict: dict.clone() })))
                }
                "keys" => {
                    struct DictKeys {
                        dict: Rc<RefCell<HashMap<String, Value>>>,
                    }
                    impl KyroCallable for DictKeys {
                        fn arity(&self) -> usize {
                            0
                        }
                        fn call(
                            &self,
                            _: &mut Interpreter,
                            _: Vec<Value>,
                        ) -> Result<Value, RuntimeError> {
                            let borrowed = self.dict.borrow();
                            let mut keys = Vec::new();
                            for key in borrowed.keys() {
                                keys.push(Value::String(key.clone()));
                            }
                            Ok(Value::List(Rc::new(RefCell::new(keys))))
                        }
                        fn name(&self) -> &str {
                            "keys"
                        }
                    }
                    Ok(Value::Callable(Rc::new(DictKeys { dict: dict.clone() })))
                }
                _ => Err(RuntimeError::Error {
                    token: name.clone(),
                    message: format!("Undefined dictionary method '{}'.", name.lexeme),
                }),
            },
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

    fn visit_this(&mut self, keyword: &Token, id: usize) -> Result<Value, RuntimeError> {
        self.look_up_variable(keyword, id)
    }

    fn visit_super(
        &mut self,
        keyword: &Token,
        method: &Token,
        id: usize,
    ) -> Result<Value, RuntimeError> {
        let distance = self.locals.get(&id).ok_or_else(|| RuntimeError::Error {
            token: keyword.clone(),
            message: "Internal error: unresolved superclass expression.".to_string(),
        })?;

        let superclass_val = Environment::get_at(self.environment.clone(), *distance, "super")
            .ok_or_else(|| RuntimeError::Error {
                token: keyword.clone(),
                message: "Internal error: superclass not found in environment.".to_string(),
            })?;

        let superclass = match superclass_val {
            Value::Class(cls) => cls,
            _ => unreachable!(),
        };

        let instance_val = Environment::get_at(self.environment.clone(), *distance - 1, "this")
            .ok_or_else(|| RuntimeError::Error {
                token: keyword.clone(),
                message: "Internal error: instance 'this' not found in environment.".to_string(),
            })?;

        let instance = match instance_val {
            Value::Instance(inst) => inst,
            _ => unreachable!(),
        };

        if let Some(superclass_method) = superclass.find_method(&method.lexeme) {
            let bound_method = superclass_method.bind(instance.clone());
            Ok(Value::Callable(Rc::new(bound_method)))
        } else {
            Err(RuntimeError::Error {
                token: method.clone(),
                message: format!("Undefined property '{}'.", method.lexeme),
            })
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
                        return Err(RuntimeError::Error {
                            token: paren.clone(),
                            message: "List index must be a number.".to_string(),
                        });
                    }
                };

                let i = num as usize;
                if num < 0.0 || i >= borrowed.len() {
                    return Err(RuntimeError::Error {
                        token: paren.clone(),
                        message: "List index out of bounds.".to_string(),
                    });
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
            _ => Err(RuntimeError::Error {
                token: paren.clone(),
                message: "Only lists and dictionaries support subscript indexing.".to_string(),
            }),
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
                        return Err(RuntimeError::Error {
                            token: paren.clone(),
                            message: "List index must be a number.".to_string(),
                        });
                    }
                };

                let i = num as usize;
                if num < 0.0 || i >= borrowed.len() {
                    return Err(RuntimeError::Error {
                        token: paren.clone(),
                        message: "List index out of bounds.".to_string(),
                    });
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
            _ => Err(RuntimeError::Error {
                token: paren.clone(),
                message: "Only lists and dictionaries support subscript assignment.".to_string(),
            }),
        }
    }
}
