use std::{iter::Peekable, process::exit};
mod expr;
mod stmt;
use crate::{ast, error::Error, lexer::TokenKind};
use logos::{Logos, Span, SpannedIter};

pub struct Parser<'a> {
    tokenkind: Peekable<SpannedIter<'a, TokenKind>>,
    cur_span: Span,
    input: &'a str,
    errors: Vec<Error>,
}

impl<'a> Parser<'a> {
    pub fn new(inp: &'a str) -> Self {
        Self {
            tokenkind: TokenKind::lexer(inp).spanned().peekable(),
            cur_span: 0..0,
            input: inp,
            errors: Vec::new(),
        }
    }

    pub fn next(&mut self) -> Option<TokenKind> {
        let kind = self.tokenkind.next().unwrap();
        self.cur_span = kind.1;
        if kind.0 != TokenKind::EOF {
            return Some(kind.0);
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> TokenKind {
        let (kind, span) = self
            .tokenkind
            .peek()
            .cloned()
            .unwrap_or((TokenKind::EOF, self.input.len()..1 - self.input.len()));
        self.cur_span = span;

        kind
    }

    pub fn span(&self) -> Span {
        self.cur_span.clone()
    }

    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    pub fn text(&mut self) -> &'a str {
        &self.input[self.span()]
    }

    pub fn consume<T>(&mut self, expected: TokenKind) -> Result<T, Error> {
        let token = self.next().unwrap();
        if expected != token {
            let err = Error::new(
                format!("Expected to consume {} but got {:#?}", expected, token),
                "E001".to_string(),
                self.span(),
            );
            self.errors.push(err.clone());
            Err(err)
        } else {
            Ok(T)
        }
    }

    pub fn consume_stmt(&mut self, expected: TokenKind) -> Option<ast::Stmt> {
        let token = self.next().unwrap();
        if expected != token {
            let err = Error::new(
                format!("Expected to consume {} but got {}", expected, token),
                "E001".to_string(),
                self.span(),
            );
            self.errors.push(err.clone());
            Some(ast::Stmt::Error(err.clone()))
        } else {
            None
        }
    }
}
