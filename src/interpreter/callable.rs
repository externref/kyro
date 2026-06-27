use crate::interpreter::{interpreter::Interpreter, runtime_error::RuntimeError, value::Value};

pub trait KyroCallable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError>;

    fn name(&self) -> &str;

    fn doc(&self) -> Option<&str> {
        None
    }
    fn parameter_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        _param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        None
    }
}
