use std::iter::Peekable;

use logos::{SpannedIter, Span, Logos};
use crate::lexer::TokenKind;

pub struct Parser<'a> {
    tokenkind: Peekable<SpannedIter<'a, TokenKind>>,
    cur_span: Span,
    input: &'a str,
}

impl<'a> Parser<'a> {
    // Utility helper methods
    pub fn new(inp: &'a str) -> Self {
        Self {
            tokenkind: TokenKind::lexer(inp).spanned().peekable(),
            cur_span: 0..0,
            input: inp,
        }
    }

    pub fn next(&mut self) -> TokenKind {
        let (kind, span) = self
            .tokenkind
            .next()
            .unwrap_or((TokenKind::EOF, self.input.len() - 1..self.input.len()));
        kind
    }

    pub fn peek(&mut self) -> TokenKind {
        let (kind, span) = self
            .tokenkind
            .peek()
            .cloned()
            .unwrap_or((TokenKind::EOF, self.span()));
        kind
    }

    pub fn span(&self) -> Span {
        self.cur_span.clone()
    }

    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    pub fn text(&self) -> &'a str {
        &self.input[self.span()]
    }
}
