use super::tokens::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
        id: usize,
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
    Variable {
        name: Token,
        id: usize,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    This {
        keyword: Token,
        id: usize,
    },
    Super {
        keyword: Token,
        method: Token,
        id: usize,
    },
    List {
        elements: Vec<Expr>,
        id: usize,
    },
    Dict {
        entries: Vec<(Expr, Expr)>,
        id: usize,
    },
    Subscript {
        object: Box<Expr>,
        index: Box<Expr>,
        paren: Token,
        id: usize,
    },
    SubscriptAssign {
        object: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
        paren: Token,
        id: usize,
    },
    Interpolate {
        parts: Vec<Expr>,
        id: usize,
    },
}

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping(&mut self, expr: &Expr) -> T;
    fn visit_literal(&mut self, value: &Literal) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_variable(&mut self, name: &Token, id: usize) -> T;
    fn visit_assign(&mut self, name: &Token, value: &Expr, id: usize) -> T;
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_call(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> T;
    fn visit_get(&mut self, object: &Expr, name: &Token) -> T;
    fn visit_set(&mut self, object: &Expr, name: &Token, value: &Expr) -> T;
    fn visit_this(&mut self, keyword: &Token, id: usize) -> T;
    fn visit_super(&mut self, keyword: &Token, method: &Token, id: usize) -> T;
    fn visit_list(&mut self, elements: &[Expr], id: usize) -> T;
    fn visit_dict(&mut self, entries: &[(Expr, Expr)], id: usize) -> T;
    fn visit_subscript(&mut self, object: &Expr, index: &Expr, paren: &Token, id: usize) -> T;
    fn visit_subscript_assign(
        &mut self,
        object: &Expr,
        index: &Expr,
        value: &Expr,
        paren: &Token,
        id: usize,
    ) -> T;
    fn visit_interpolate(&mut self, parts: &[Expr], id: usize) -> T;
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
            Expr::Variable { name, id } => visitor.visit_variable(name, *id),
            Expr::Assign { name, value, id } => visitor.visit_assign(name, value, *id),
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
            Expr::This { keyword, id } => visitor.visit_this(keyword, *id),
            Expr::Super {
                keyword,
                method,
                id,
            } => visitor.visit_super(keyword, method, *id),
            Expr::List { elements, id } => visitor.visit_list(elements, *id),
            Expr::Dict { entries, id } => visitor.visit_dict(entries, *id),
            Expr::Subscript {
                object,
                index,
                paren,
                id,
            } => visitor.visit_subscript(object, index, paren, *id),
            Expr::SubscriptAssign {
                object,
                index,
                value,
                paren,
                id,
            } => visitor.visit_subscript_assign(object, index, value, paren, *id),
            Expr::Interpolate { parts, id } => visitor.visit_interpolate(parts, *id),
        }
    }
}
