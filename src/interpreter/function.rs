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
    pub doc: Option<String>,
}

impl KyroFunction {
    pub fn new(
        declaration: Stmt,
        closure: EnvRef,
        is_initializer: bool,
        doc: Option<String>,
    ) -> Self {
        Self {
            declaration,
            closure,
            is_initializer,
            doc,
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<KyroInstance>>) -> Self {
        let mut environment = Environment::from_enclosing(self.closure.clone());
        environment.define("this".to_string(), Value::Instance(instance));
        Self::new(
            self.declaration.clone(),
            Rc::new(RefCell::new(environment)),
            self.is_initializer,
            self.doc.clone(),
        )
    }
}

impl KyroCallable for KyroFunction {
    fn arity(&self) -> usize {
        match &self.declaration {
            Stmt::Function { params, .. } => {
                params.iter().filter(|p| p.default_value.is_none()).count()
            }
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
                    environment.define(param.name.lexeme.clone(), argument);
                }

                let result = interpreter.execute_block(body, Rc::new(RefCell::new(environment)));

                let return_val = match result {
                    Ok(_) => Value::Nil,
                    Err(RuntimeError::Return(value)) => value,
                    Err(error) => return Err(error),
                };

                if self.is_initializer {
                    Ok(self.closure.borrow().get("this").unwrap_or(Value::Nil))
                } else {
                    Ok(return_val)
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

    fn doc(&self) -> Option<&str> {
        self.doc.as_deref()
    }

    fn parameter_names(&self) -> Vec<String> {
        match &self.declaration {
            Stmt::Function { params, .. } => params.iter().map(|p| p.name.lexeme.clone()).collect(),
            _ => unreachable!(),
        }
    }

    fn default_value(
        &self,
        interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        match &self.declaration {
            Stmt::Function { params, .. } => {
                let param = params.iter().find(|p| p.name.lexeme == param_name)?;
                let default_expr = param.default_value.as_ref()?;

                let previous_env = interpreter.environment.clone();
                interpreter.environment = self.closure.clone();
                let result = interpreter.interpret(default_expr);

                interpreter.environment = previous_env;

                Some(result)
            }
            _ => unreachable!(),
        }
    }
}
