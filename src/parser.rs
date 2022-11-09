use crate::lexer::Lexer;
use crate::tokens::TokenKind;
use logos::{Logos, SpannedIter};
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }
}
