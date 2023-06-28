use std::fmt;

use crate::{error::Error, lexer::TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Struct {
        name: Result<Type, Error>,
        members: Vec<(String, Result<Type, Error>)>,
    },
    Function {
        name: String,
        parameters: Vec<(String, Result<Type, Error>)>,
        body: Vec<Result<Stmt, Error>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub name: String,
    pub generics: Vec<Result<Type, Error>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        var_name: String,
        value: Box<Result<Expr, Error>>,
    },
    Assignment {
        var_name: String,
        value: Box<Result<Expr, Error>>,
    },
    IfStmt {
        condition: Box<Result<Expr, Error>>,
        body: Vec<Result<Stmt, Error>>,
        else_stmt: Option<Box<Result<Stmt, Error>>>,
    },
    Block {
        stmts: Vec<Result<Stmt, Error>>,
    },

    Error(Error),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Lit),
    Ident(String),
    FnCall {
        fn_name: String,
        args: Vec<Result<Expr, Error>>,
    },
    PrefixOp {
        op: TokenKind,
        expr: Box<Result<Expr, Error>>,
    },
    InfixOp {
        op: TokenKind,
        lhs: Box<Result<Expr, Error>>,
        rhs: Box<Result<Expr, Error>>,
    },
    PostfixOp {
        op: TokenKind,
        expr: Box<Result<Expr, Error>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(usize),
    Float(f64),
    Str(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(lit) => write!(f, "{}", lit),
            Expr::Ident(name) => write!(f, "{}", name),
            Expr::FnCall { fn_name, args } => {
                write!(f, "{}(", fn_name)?;
                for arg in args {
                    write!(f, "{:#?},", arg)?;
                }
                write!(f, ")")
            }
            Expr::PrefixOp { op, expr } => write!(f, "({:#?} {:#?})", op, expr),
            Expr::InfixOp { op, lhs, rhs } => write!(f, "({:#?} {:#?} {:#?})", lhs, op, rhs),
            Expr::PostfixOp { op, expr } => write!(f, "({:#?} {:#?})", expr, op),
        }
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Int(i) => write!(f, "{}", i),
            Lit::Float(fl) => write!(f, "{}", fl),
            Lit::Str(s) => write!(f, r#""{}""#, s),
        }
    }
}
