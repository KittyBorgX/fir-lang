use std::iter::Peekable;

use logos::{Logos, Span, SpannedIter};

use crate::tokens::TokenKind;
struct Token {
    kind: TokenKind,
    span: Span,
}

pub struct Lexer<'a> {
    tokenkind: Peekable<SpannedIter<'a, TokenKind>>,
    token: Token,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(inp: &'a str) -> Self {
        Self {
            tokenkind: TokenKind::lexer(inp).spanned().peekable(),
            token: Token {
                kind: TokenKind::Error,
                span: 0..0,
            },
            input: inp,
        }
    }

    pub fn next(&mut self) -> TokenKind {
        let (kind, span) = self
            .tokenkind
            .next()
            .unwrap_or((TokenKind::EOF, self.input.len() - 1..self.input.len()));
        self.token = Token { kind, span };
        kind
    }

    pub fn peek(&mut self) -> TokenKind {
        let (kind, span) = self
            .tokenkind
            .peek()
            .cloned()
            .unwrap_or((TokenKind::EOF, self.span()));
        self.token = Token { kind: kind, span };
        kind
    }

    pub fn span(&self) -> Span {
        self.token.span.clone()
    }

    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }
}
