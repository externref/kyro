use std::{cell::RefCell, rc::Rc};

use crate::{
    interpreter::{
        callable::KyroCallable,
        environment::{EnvRef, Environment},
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
}

impl KyroFunction {
    pub fn new(declaration: Stmt, closure: EnvRef) -> Self {
        Self {
            declaration,
            closure,
        }
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
                    Ok(_) => Ok(Value::Nil),
                    Err(RuntimeError::Return(value)) => Ok(value),
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
