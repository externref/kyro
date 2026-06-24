use std::{cell::RefCell, rc::Rc};

use crate::{
    interpreter::{
        callable::KyroCallable,
        environment::{EnvRef, Environment},
        instance::KyroInstance,
        interpreter::Interpreter,
        runtime_error::RuntimeError,
        value::Value,
    },
    parser::stmt::Stmt,
};

#[derive(Clone)]
pub struct KyroFunction {
    pub declaration: Stmt,
    pub closure: EnvRef,
    pub is_initializer: bool,
}

impl KyroFunction {
    pub fn new(declaration: Stmt, closure: EnvRef, is_initializer: bool) -> Self {
        Self {
            declaration,
            closure,
            is_initializer,
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<KyroInstance>>) -> Self {
        let mut environment = Environment::from_enclosing(self.closure.clone());
        environment.define("this".to_string(), Value::Instance(instance));
        Self::new(
            self.declaration.clone(),
            Rc::new(RefCell::new(environment)),
            self.is_initializer,
        )
    }
}

impl KyroCallable for KyroFunction {
    fn arity(&self) -> usize {
        match &self.declaration {
            Stmt::Function { params, .. } => params.len(),
            _ => unreachable!(),
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let mut environment = Environment::from_enclosing(self.closure.clone());
        match &self.declaration {
            Stmt::Function { params, body, .. } => {
                for (param, argument) in params.iter().zip(arguments) {
                    environment.define(param.lexeme.clone(), argument);
                }

                let result = interpreter.execute_block(body, Rc::new(RefCell::new(environment)));

                match result {
                    Ok(_) => {
                        if self.is_initializer {
                            Ok(self.closure.borrow().get("this").unwrap_or(Value::Nil))
                        } else {
                            Ok(Value::Nil)
                        }
                    }
                    Err(RuntimeError::Return(value)) => {
                        if self.is_initializer {
                            Ok(self.closure.borrow().get("this").unwrap_or(Value::Nil))
                        } else {
                            Ok(value)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            _ => unreachable!(),
        }
    }

    fn name(&self) -> &str {
        match &self.declaration {
            Stmt::Function { name, .. } => &name.lexeme,
            _ => unreachable!(),
        }
    }
}
