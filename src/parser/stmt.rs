use super::{expr::Expr, tokens::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),

    Print(Expr),

    Var {
        name: Token,
        initializer: Option<Expr>,
    },

    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Class {
        name: Token,
        methods: Vec<Stmt>,
    },
}
