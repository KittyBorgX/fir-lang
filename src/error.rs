use logos::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    msg: String,
    code: String,
    span: Span,
}
impl Error {
    pub fn new(msg: String, code: String, span: Span) -> Self {
        Self { msg, code, span }
    }
}
