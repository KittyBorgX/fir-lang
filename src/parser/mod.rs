use std::iter::Peekable;
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

    pub fn next(&mut self) -> TokenKind {
        let (kind, span) = self
            .tokenkind
            .next()
            .unwrap_or((TokenKind::EOF, self.input.len()..self.input.len()));
        self.cur_span = span;
        kind
    }

    pub fn peek(&mut self) -> TokenKind {
        let (kind, span) = self
            .tokenkind
            .peek()
            .cloned()
            .unwrap_or((TokenKind::EOF, self.input.len()..self.input.len()));
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

    pub fn consume(&mut self, expected: TokenKind) -> Result<(), Error> {
        let token = self.next();
        if expected != token {
            let err = Error::new(
                format!("Expected to consume {} but got {}", expected, token),
                "E001".to_string(),
                self.span(),
            );
            self.errors.push(err.clone());
            return Err(err.clone());
        } else {
            return Ok(());
        }
    }

    pub fn consume_stmt(&mut self, expected: TokenKind) -> Option<ast::Stmt> {
        let token = self.next();
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
