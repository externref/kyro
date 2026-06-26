use super::{expr::Expr, tokens::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Echo(Expr),
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
        doc: Option<String>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Class {
        name: Token,
        super_class: Option<Expr>,
        methods: Vec<Stmt>,
        doc: Option<String>,
    },
    TryCatch {
        try_branch: Box<Stmt>,
        exception_var: Token,
        catch_branch: Box<Stmt>,
    },
    Throw {
        keyword: Token,
        value: Expr,
    },
    For {
        initializer: Option<Box<Stmt>>,
        condition: Expr,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },
    Break {
        keyword: Token,
    },
    Continue {
        keyword: Token,
    },
}
