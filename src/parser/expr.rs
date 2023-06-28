use crate::{ast, lexer::TokenKind};

use super::Parser;

impl<'a> Parser<'a> {
    #[inline]
    pub fn expression(&mut self) -> ast::Expr {
        self.parse_expression(0)
    }

    pub fn parse_expression(&mut self, binding_power: u8) -> ast::Expr {
        let mut lhs = match self.peek() {
            lit @ TokenKind::Int | lit @ TokenKind::Float | lit @ TokenKind::String => {
                let literal_text = {
                    // if `peek` is not `TokenKind::EOF]`, then there must be a next token
                    let _literal_token = self.next();
                    self.text()
                };
                let lit = match lit {
                    TokenKind::Int => ast::Lit::Int(
                        literal_text
                            .parse()
                            .expect(&format!("invalid integer literal: `{}`", literal_text)),
                    ),
                    TokenKind::Float => ast::Lit::Float(literal_text.parse().expect(&format!(
                        "invalid floating point literal: `{}`",
                        literal_text
                    ))),
                    TokenKind::String => {
                        ast::Lit::Str(literal_text[1..(literal_text.len() - 1)].to_string())
                    }
                    _ => unreachable!(),
                };
                ast::Expr::Literal(lit)
            }
            TokenKind::Ident => {
                let name = {
                    let _ident_token = self.next();
                    self.text().to_string()
                };
                if !self.at(TokenKind::LParen) {
                    // plain identifier
                    ast::Expr::Ident(name)
                } else {
                    //  function call
                    let mut args = Vec::new();
                    self.consume(TokenKind::LParen);
                    while !self.at(TokenKind::RParen) {
                        let arg = self.parse_expression(0);
                        args.push(arg);
                        if self.at(TokenKind::Comma) {
                            self.consume(TokenKind::Comma);
                        }
                    }
                    self.consume(TokenKind::RParen);
                    ast::Expr::FnCall {
                        fn_name: name,
                        args,
                    }
                }
            }
            TokenKind::LParen => {
                // There is no AST node for grouped expressions.
                // Parentheses just influence the tree structure.
                self.consume(TokenKind::LParen);
                let expr = self.parse_expression(0);
                self.consume(TokenKind::RParen);
                expr
            }
            op @ TokenKind::Plus | op @ TokenKind::Minus | op @ TokenKind::Bang => {
                self.consume(op);
                let ((), right_binding_power) = op.prefix_binding_power();
                let expr = self.parse_expression(right_binding_power);
                ast::Expr::PrefixOp {
                    op,
                    expr: Box::new(expr),
                }
            }
            kind => {
                panic!("Unknown start of expression: `{}`", kind);
            }
        };
        loop {
            let op = match self.peek() {
                op @ TokenKind::Plus
                | op @ TokenKind::Minus
                | op @ TokenKind::Times
                | op @ TokenKind::Slash
                | op @ TokenKind::Caret
                | op @ TokenKind::Eqq
                | op @ TokenKind::Neq
                | op @ TokenKind::And
                | op @ TokenKind::Or
                | op @ TokenKind::LAngle
                | op @ TokenKind::Leq
                | op @ TokenKind::RAngle
                | op @ TokenKind::Geq
                | op @ TokenKind::Bang => op,
                TokenKind::EOF => break,
                TokenKind::RParen | TokenKind::RBrace | TokenKind::Comma | TokenKind::SemiColon => {
                    break
                }
                kind => panic!("Unknown operator: `{}`", kind),
            };

            if let Some((left_binding_power, ())) = op.postfix_binding_power() {
                if left_binding_power < binding_power {
                    // previous operator has higher binding power than new one --> end of expression
                    break;
                }

                self.consume(op);
                // no recursive call here, because we have already parsed our operand `lhs`
                lhs = ast::Expr::PostfixOp {
                    op,
                    expr: Box::new(lhs),
                };
                // parsed an operator --> go round the loop again
                continue;
            }

            if let Some((left_binding_power, right_binding_power)) = op.infix_binding_power() {
                if left_binding_power < binding_power {
                    // previous operator has higher binding power than new one --> end of expression
                    break;
                }

                self.consume(op);
                let rhs = self.parse_expression(right_binding_power);
                lhs = ast::Expr::InfixOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
                // parsed an operator --> go round the loop again
                continue;
            }

            break; // Not an operator --> end of expression
        }

        lhs
    }
}

pub trait Operator {
    /// Prefix operators bind their operand to the right.
    fn prefix_binding_power(&self) -> ((), u8);

    /// Infix operators bind two operands, lhs and rhs.
    fn infix_binding_power(&self) -> Option<(u8, u8)>;

    /// Postfix operators bind their operand to the left.
    fn postfix_binding_power(&self) -> Option<(u8, ())>;
}

impl Operator for TokenKind {
    fn prefix_binding_power(&self) -> ((), u8) {
        match self {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Bang => ((), 51),
            // Prefixes are the only operators we have already seen
            // when we call this, so we know the token must be
            // one of the above
            _ => unreachable!("Not a prefix operator: {:?}", self),
        }
    }

    fn infix_binding_power(&self) -> Option<(u8, u8)> {
        let result = match self {
            TokenKind::Or => (1, 2),
            TokenKind::And => (3, 4),
            TokenKind::Eqq | TokenKind::Neq => (5, 6),
            TokenKind::LAngle | TokenKind::RAngle | TokenKind::Leq | TokenKind::Geq => (7, 8),
            TokenKind::Plus | TokenKind::Minus => (9, 10),
            TokenKind::Times | TokenKind::Slash => (11, 12),
            TokenKind::Caret => (22, 21), // <- This binds stronger to the left!
            _ => return None,
        };
        Some(result)
    }

    fn postfix_binding_power(&self) -> Option<(u8, ())> {
        let result = match self {
            TokenKind::Bang => (101, ()),
            _ => return None,
        };
        Some(result)
    }
}
