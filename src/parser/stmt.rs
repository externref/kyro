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

use super::{expr::Expr, tokens::Token};

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Token,
    pub default_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Echo(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    VarDestructure {
        names: Vec<Token>,
        is_dict: bool,
        initializer: Expr,
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
    For {
        initializer: Option<Box<Stmt>>,
        condition: Expr,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },
    ForIn {
        var_name: Token,
        collection: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<Parameter>,
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
    Break {
        keyword: Token,
    },
    Continue {
        keyword: Token,
    },
}
