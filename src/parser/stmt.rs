use std::process::exit;

use super::Parser;
use crate::ast::Item;
use crate::error::Error;
use crate::lexer::TokenKind;
use crate::{ast, error};

impl<'a> Parser<'a> {
    pub fn parse(input: &str) -> Vec<ast::Item> {
        let mut parser = Parser::new(input);
        parser.file()
    }

    pub fn file(&mut self) -> Vec<ast::Item> {
        let mut items = Vec::new();
        while !self.at(TokenKind::EOF) {
            let item = self.item();
            dbg!(item.clone());
            items.push(item);
        }
        items
    }

    pub fn item(&mut self) -> ast::Item {
        dbg!(self.peek());
        match self.peek() {
            TokenKind::KwFn => {
                self.consume::<ast::Item>(TokenKind::KwFn);
                println!("here");
                let mut parameters = Vec::new();

                let ident = self.next().unwrap();
                if ident != TokenKind::Ident {
                    self.errors.push(Error::new(
                        format!(
                            "Expected identifier as function name, but found `{:#?}`",
                            ident
                        ),
                        "E001".to_string(),
                        self.span(),
                    ))
                }
                let name = self.text().to_string();

                self.consume::<ast::Item>(TokenKind::LParen);
                while !self.at(TokenKind::RParen) {
                    let _parameter_ident = self.next().unwrap();

                    if ident != TokenKind::Ident {
                        self.errors.push(Error::new(
                            format!(
                                "Expected identifier as function parameter, but found `{:#?}`",
                                ident
                            ),
                            "E001".to_string(),
                            self.span(),
                        ))
                    }
                    let parameter_name = self.text().to_string();
                    self.consume::<ast::Item>(TokenKind::Colon);
                    let parameter_type = self.type_();
                    parameters.push((parameter_name, parameter_type));
                    if self.at(TokenKind::Comma) {
                        self.consume::<ast::Item>(TokenKind::Comma);
                    }
                }
                println!("here2");

                self.consume::<ast::Item>(TokenKind::RParen);

                assert!(
                    self.at(TokenKind::LBrace),
                    "Expected a block after function header"
                );
                let body = match self.statement() {
                    Ok(ast::Stmt::Block { stmts }) => stmts,
                    _ => unreachable!(),
                };
                println!("here3");

                ast::Item::Function {
                    name,
                    parameters,
                    body,
                }
            }
            TokenKind::KwStruct => {
                self.consume::<ast::Item>(TokenKind::KwStruct);
                let mut members = Vec::new();
                let name = self.type_();
                self.consume::<ast::Item>(TokenKind::LBrace);
                while !self.at(TokenKind::RBrace) {
                    let member_ident = self.next().unwrap();
                    if member_ident != TokenKind::Ident {
                        self.errors.push(Error::new(
                            format!(
                                "Expected identifier as struct member, but found `{:#?}`",
                                member_ident
                            ),
                            "E001".to_string(),
                            self.span(),
                        ))
                    }
                    let member_name = self.text().to_string();
                    self.consume::<ast::Item>(TokenKind::Colon);
                    let member_type = self.type_();
                    members.push((member_name, member_type));
                    if self.at(TokenKind::Comma) {
                        self.consume::<ast::Item>(TokenKind::Comma);
                    }
                }
                self.consume::<ast::Item>(TokenKind::RBrace);
                ast::Item::Struct { name, members }
            }
            kind => panic!("Unknown start of item: `{}`", kind),
        }
    }

    pub fn type_(&mut self) -> ast::Type {
        let ident = self.next().unwrap();
        if ident != TokenKind::Ident {
            self.errors.push(Error::new(
                format!(
                    "Expected identifier as start of type, but found `{:#?}`",
                    ident
                ),
                "E001".to_string(),
                self.span(),
            ))
        }

        let name = self.text().to_string();

        let mut generics = Vec::new();

        if self.at(TokenKind::LAngle) {
            self.consume::<ast::Type>(TokenKind::LAngle);
            while !self.at(TokenKind::RAngle) {
                // Generic parameters are also types
                let generic = self.type_();
                generics.push(generic);
                if self.at(TokenKind::Comma) {
                    self.consume::<ast::Type>(TokenKind::Comma);
                }
            }
            self.consume::<ast::Type>(TokenKind::RAngle);
        }

        ast::Type { name, generics }
    }

    pub fn statement(&mut self) -> Result<ast::Stmt, error::Error> {
        dbg!(self.peek());
        match self.peek() {
            TokenKind::KwLet => {
                self.consume::<ast::Stmt>(TokenKind::KwLet);
                let ident = self.next().unwrap();
                if ident != TokenKind::Ident {
                    let err = Error::new(
                        format!("Expected identifier after `let`, but found `{:#?}`", ident),
                        "E001".to_string(),
                        self.span().clone(),
                    );
                    self.errors.push(err.clone());
                    ast::Stmt::Error(err.clone());
                }
                let name = self.text().to_string();
                self.consume_stmt(TokenKind::Eq);
                let value = self.expression();
                let _ = self.consume::<ast::Stmt>(TokenKind::SemiColon);
                Ok(ast::Stmt::Let {
                    var_name: name,
                    value: Box::new(value),
                })
            }
            TokenKind::Ident => {
                let ident = self.next().unwrap();
                let name = self.text().to_string();
                self.consume::<ast::Stmt>(TokenKind::Eq);
                let value = self.expression();
                self.consume::<ast::Stmt>(TokenKind::SemiColon);
                Ok(ast::Stmt::Assignment {
                    var_name: name,
                    value: Box::new(value),
                })
            }
            TokenKind::KwIf => {
                self.consume::<ast::Stmt>(TokenKind::KwIf);
                self.consume::<ast::Stmt>(TokenKind::LParen);
                let condition = self.expression();
                self.consume::<ast::Stmt>(TokenKind::RParen);

                assert!(
                    self.at(TokenKind::LBrace),
                    "Expected a block after `if` statement"
                );
                let body = self.statement();
                let body = match body {
                    Ok(ast::Stmt::Block { stmts }) => stmts,
                    _ => unreachable!(),
                };

                let else_stmt = if self.at(TokenKind::KwElse) {
                    self.consume::<ast::Stmt>(TokenKind::KwElse);
                    assert!(
                        self.at(TokenKind::KwIf) || self.at(TokenKind::LBrace),
                        "Expected a block or an `if` after `else` statement"
                    );
                    Some(Box::new(self.statement()))
                } else {
                    None
                };

                Ok(ast::Stmt::IfStmt {
                    condition: Box::new(condition),
                    body,
                    else_stmt,
                })
            }
            TokenKind::LBrace => {
                self.consume::<ast::Stmt>(TokenKind::LBrace);
                let mut stmts = Vec::new();
                while !self.at(TokenKind::RBrace) {
                    let stmt = self.statement();
                    stmts.push(stmt);
                }
                self.consume::<ast::Stmt>(TokenKind::RBrace);
                Ok(ast::Stmt::Block { stmts })
            }

            // TokenKind::EOF => {
            //     print!("hit eof lol");
            //     break 'block Ok(ast::Stmt::EOF;
            // }
            kind => Ok(ast::Stmt::Error(Error::new(
                format!("Unknown start of statement {}", kind),
                "E002".to_string(),
                self.span(),
            ))),
        }
    }
}
