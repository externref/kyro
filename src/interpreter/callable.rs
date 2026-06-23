use crate::interpreter::{interpreter::Interpreter, runtime_error::RuntimeError, value::Value};

pub trait KyroCallable {
    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError>;

    fn name(&self) -> &str;
}
