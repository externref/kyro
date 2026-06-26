use crate::interpreter::value::Value;
use crate::parser::tokens::Token;

pub enum RuntimeError {
    Error { token: Token, value: Value },
    Return(Value),
    Break,
    Continue,
}

impl RuntimeError {
    pub fn new(token: Token, message: impl Into<String>) -> Self {
        Self::Error {
            token,
            value: Value::String(message.into()),
        }
    }
}
