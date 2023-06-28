use super::Parser;
use crate::ast;
use crate::error::Error;
use crate::lexer::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse(input: &str) -> Vec<ast::Item> {
        let mut parser = Parser::new(input);
        parser.file()
    }

    pub fn file(&mut self) -> Vec<ast::Item> {
        let mut items = Vec::new();
        while !self.at(TokenKind::EOF) {
            let item = self.item();
            items.push(item);
        }
        items
    }

    pub fn item(&mut self) -> ast::Item {
        match self.peek() {
            TokenKind::KwFn => {
                self.consume(TokenKind::KwFn);
                let mut parameters = Vec::new();

                let ident = self.next();
                if ident != TokenKind::Ident {
                    self.errors.push(Error::new(
                        format!(
                            "Expected identifier as function name, but found `{}`",
                            ident
                        ),
                        "E001".to_string(),
                        self.span(),
                    ))
                }
                let name = self.text().to_string();

                self.consume(TokenKind::LParen);
                while !self.at(TokenKind::RParen) {
                    let _parameter_ident = self.next();

                    if ident != TokenKind::Ident {
                        self.errors.push(Error::new(
                            format!(
                                "Expected identifier as function parameter, but found `{}`",
                                ident
                            ),
                            "E001".to_string(),
                            self.span(),
                        ))
                    }
                    let parameter_name = self.text().to_string();
                    self.consume(TokenKind::Colon);
                    let parameter_type = self.type_();
                    parameters.push((parameter_name, parameter_type));
                    if self.at(TokenKind::Comma) {
                        self.consume(TokenKind::Comma);
                    }
                }
                self.consume(TokenKind::RParen);

                assert!(
                    self.at(TokenKind::LBrace),
                    "Expected a block after function header"
                );
                let body = match self.statement() {
                    ast::Stmt::Block { stmts } => stmts,
                    _ => unreachable!(),
                };

                ast::Item::Function {
                    name,
                    parameters,
                    body,
                }
            }
            TokenKind::KwStruct => {
                self.consume(TokenKind::KwStruct);
                let mut members = Vec::new();
                let name = self.type_();
                self.consume(TokenKind::LBrace);
                while !self.at(TokenKind::RBrace) {
                    let member_ident = self.next();
                    if member_ident != TokenKind::Ident {
                        self.errors.push(Error::new(
                            format!(
                                "Expected identifier as struct member, but found `{}`",
                                member_ident
                            ),
                            "E001".to_string(),
                            self.span(),
                        ))
                    }
                    let member_name = self.text().to_string();
                    self.consume(TokenKind::Colon);
                    let member_type = self.type_();
                    members.push((member_name, member_type));
                    if self.at(TokenKind::Comma) {
                        self.consume(TokenKind::Comma);
                    }
                }
                self.consume(TokenKind::RBrace);
                ast::Item::Struct { name, members }
            }
            kind => panic!("Unknown start of item: `{}`", kind),
        }
    }

    pub fn type_(&mut self) -> ast::Type {
        let ident = self.next();
        if ident != TokenKind::Ident {
            self.errors.push(Error::new(
                format!(
                    "Expected identifier as start of type, but found `{}`",
                    ident
                ),
                "E001".to_string(),
                self.span(),
            ))
        }

        let name = self.text().to_string();

        let mut generics = Vec::new();

        if self.at(TokenKind::LAngle) {
            self.consume(TokenKind::LAngle);
            while !self.at(TokenKind::RAngle) {
                // Generic parameters are also types
                let generic = self.type_();
                generics.push(generic);
                if self.at(TokenKind::Comma) {
                    self.consume(TokenKind::Comma);
                }
            }
            self.consume(TokenKind::RAngle);
        }

        ast::Type { name, generics }
    }

    pub fn statement(&mut self) -> ast::Stmt {
        match self.peek() {
            TokenKind::KwLet => {
                self.consume(TokenKind::KwLet);
                let ident = self.next();
                if ident != TokenKind::Ident {
                    let err = Error::new(
                        format!("Expected identifier after `let`, but found `{}`", ident),
                        "E001".to_string(),
                        self.span().clone(),
                    );
                    self.errors.push(err.clone());
                    ast::Stmt::Error(err.clone());
                }
                let name = self.text().to_string();
                self.consume_stmt(TokenKind::Eq);
                let value = self.expression();
                let _ = self.consume(TokenKind::SemiColon);
                ast::Stmt::Let {
                    var_name: name,
                    value: Box::new(value),
                }
            }
            TokenKind::Ident => {
                let ident = self.next();
                let name = self.text().to_string();
                self.consume(TokenKind::Eq);
                let value = self.expression();
                self.consume(TokenKind::SemiColon);
                ast::Stmt::Assignment {
                    var_name: name,
                    value: Box::new(value),
                }
            }
            TokenKind::KwIf => {
                self.consume(TokenKind::KwIf);
                self.consume(TokenKind::LParen);
                let condition = self.expression();
                self.consume(TokenKind::RParen);

                assert!(
                    self.at(TokenKind::LBrace),
                    "Expected a block after `if` statement"
                );
                let body = self.statement();
                let body = match body {
                    ast::Stmt::Block { stmts } => stmts,
                    _ => unreachable!(),
                };

                let else_stmt = if self.at(TokenKind::KwElse) {
                    self.consume(TokenKind::KwElse);
                    assert!(
                        self.at(TokenKind::KwIf) || self.at(TokenKind::LBrace),
                        "Expected a block or an `if` after `else` statement"
                    );
                    Some(Box::new(self.statement()))
                } else {
                    None
                };

                ast::Stmt::IfStmt {
                    condition: Box::new(condition),
                    body,
                    else_stmt,
                }
            }
            TokenKind::LBrace => {
                self.consume(TokenKind::LBrace);
                let mut stmts = Vec::new();
                while !self.at(TokenKind::RBrace) {
                    let stmt = self.statement();
                    stmts.push(stmt);
                }
                self.consume(TokenKind::RBrace);
                ast::Stmt::Block { stmts }
            }
            kind => ast::Stmt::Error(Error::new(
                format!("Unknown start of statement {}", kind),
                "E002".to_string(),
                self.span(),
            )),
        }
    }
}
