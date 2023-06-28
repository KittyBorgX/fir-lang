use std::fmt;

use crate::{error::Error, lexer::TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Struct {
        name: Type,
        members: Vec<(String, Type)>,
    },
    Function {
        name: String,
        parameters: Vec<(String, Type)>,
        body: Vec<Stmt>,
    },
    Error(Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub name: String,
    pub generics: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        var_name: String,
        value: Box<Expr>,
    },
    Assignment {
        var_name: String,
        value: Box<Expr>,
    },
    IfStmt {
        condition: Box<Expr>,
        body: Vec<Stmt>,
        else_stmt: Option<Box<Stmt>>,
    },
    Block {
        stmts: Vec<Stmt>,
    },

    Error(Error),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Lit),
    Ident(String),
    FnCall {
        fn_name: String,
        args: Vec<Expr>,
    },
    PrefixOp {
        op: TokenKind,
        expr: Box<Expr>,
    },
    InfixOp {
        op: TokenKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    PostfixOp {
        op: TokenKind,
        expr: Box<Expr>,
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
                    write!(f, "{},", arg)?;
                }
                write!(f, ")")
            }
            Expr::PrefixOp { op, expr } => write!(f, "({} {})", op, expr),
            Expr::InfixOp { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::PostfixOp { op, expr } => write!(f, "({} {})", expr, op),
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
