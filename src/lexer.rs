use core::fmt;

use logos::Logos;
#[derive(Logos, Debug, Clone, PartialEq, Copy)]
pub enum TokenKind {
    // Single character operators
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(";")]
    SemiColon,
    #[token("^")]
    Caret,
    #[token("=")]
    Eq,
    #[token("_")]
    Under,
    // Binary Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("/")]
    Slash,

    // Logical Operators
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Bang,

    // Relational Operators
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,
    #[token("==")]
    Eqq,
    #[token("!=")]
    #[token("<>")]
    Neq,
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,

    // Brackets
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    // Literals
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,

    #[regex(r#"//[^\n]*\n"#, logos::skip)]
    LineComment,

    #[regex(r#"\d+"#, priority = 2)]
    Int,

    #[regex(r#"((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#)]
    Float,

    // Misc
    #[regex(r#"[A-Za-z]([A-Za-z]|_|\d)*"#)]
    Ident,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    WhiteSpace,

    // Keywords
    #[token("let")]
    KwLet,
    #[token("if")]
    KwIf,
    #[token("else")]
    KwElse,
    #[token("fn")]
    KwFn,

    #[error]
    Error,

    EOF,
}

// TODO: Change the symbols to words for the error messages in the future:
// Example: change TokenKind::Dot => "." to TokenKind::Dot => "dot"

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Single characters
                TokenKind::Dot => ".",
                TokenKind::Colon => ":",
                TokenKind::Comma => ",",
                TokenKind::SemiColon => ";",
                TokenKind::Caret => "^",
                TokenKind::Eq => "=",
                TokenKind::Under => "_",

                // Binary ops
                TokenKind::Plus => "+",
                TokenKind::Minus => "-",
                TokenKind::Times => "*",
                TokenKind::Slash => "/",

                // Logical ops
                TokenKind::And => "&&",
                TokenKind::Or => "||",
                TokenKind::Bang => "!",

                // Relational ops
                TokenKind::LAngle => "<",
                TokenKind::RAngle => ">",
                TokenKind::Eqq => "==",
                TokenKind::Neq => "!=",
                TokenKind::Geq => ">=",
                TokenKind::Leq => "<=",

                // Brackets
                TokenKind::LParen => "(",
                TokenKind::RParen => ")",
                TokenKind::LSquare => "[",
                TokenKind::RSquare => "]",
                TokenKind::LBrace => "{",
                TokenKind::RBrace => "}",

                // Literals
                TokenKind::String => "String",
                TokenKind::LineComment => "comment",
                TokenKind::Int => "an integer",
                TokenKind::Float => "a floatinf point literal",

                // Misc
                TokenKind::Ident => "an identifier",
                TokenKind::WhiteSpace => "<WS>",

                // Keywords
                TokenKind::KwLet => "let",
                TokenKind::KwIf => "if",
                TokenKind::KwElse => "else",
                TokenKind::KwFn => "fn",

                TokenKind::Error => "ERROR",
                TokenKind::EOF => "<EOF>",
            }
        )
    }
}
