use crate::{interpreter::value::Value, parser::tokens::Token};

pub enum RuntimeError {
    Error { token: Token, message: String },

    Return(Value),
}
impl RuntimeError {
    pub fn new(token: Token, message: impl Into<String>) -> Self {
        Self::Error {
            token,
            message: message.into(),
        }
    }
}
