use crate::interpreter::{runtime_error::RuntimeError, value::Value};

use super::tokens::{Literal, Token};
#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },

    Grouping(Box<Expr>),

    Literal(Literal),
    Variable(Token),
    Get {
        object: Box<Expr>,
        name: Token,
    },

    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
}
pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping(&mut self, expr: &Expr) -> T;
    fn visit_literal(&mut self, value: &Literal) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_variable(&mut self, name: &Token) -> T;
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_call(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> T;
    fn visit_get(&mut self, object: &Expr, name: &Token) -> T;
    fn visit_set(&mut self, object: &Expr, name: &Token, value: &Expr) -> T;
}
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),

            Expr::Grouping(expr) => visitor.visit_grouping(expr),

            Expr::Literal(value) => visitor.visit_literal(value),

            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Variable(name) => visitor.visit_variable(name),
            Expr::Assign { name, value } => visitor.visit_assign(name, value),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical(left, operator, right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call(callee, paren, arguments),
            Expr::Get { object, name } => visitor.visit_get(object, name),
            Expr::Set {
                object,
                name,
                value,
            } => visitor.visit_set(object, name, value),
        }
    }
}
