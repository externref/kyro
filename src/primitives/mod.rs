pub mod dict;
pub mod list;
pub mod number;
pub mod string;

pub use dict::get_dict_method;
pub use list::get_list_method;
pub use number::get_number_method;
pub use string::get_string_method;

use crate::interpreter::{
    callable::KyroCallable, interpreter::Interpreter, runtime_error::RuntimeError, value::Value,
};

pub struct PrimitiveMethod {
    name: &'static str,
    arity: usize,
    method: Box<dyn Fn(&mut Interpreter, Vec<Value>) -> Result<Value, RuntimeError>>,
}

impl PrimitiveMethod {
    pub fn new<F>(name: &'static str, arity: usize, method: F) -> Self
    where
        F: Fn(&mut Interpreter, Vec<Value>) -> Result<Value, RuntimeError> + 'static,
    {
        Self {
            name,
            arity,
            method: Box::new(method),
        }
    }
}

impl KyroCallable for PrimitiveMethod {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        (self.method)(interpreter, arguments)
    }

    fn name(&self) -> &str {
        self.name
    }
}
