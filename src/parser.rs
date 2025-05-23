use std::ops::{Deref, DerefMut};

use crate::{ast::{Expression, PrefixOp, Span, Statement}, lexer::{Lexer, Token, TokenKind}};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    index: usize
}


impl<'a> Deref for Parser<'a> {
    type Target = Lexer<'a>;

    fn deref(&self) -> &Self::Target {
        &self.lexer
    }
}

impl<'a> DerefMut for Parser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lexer
    }
    
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token { 
                kind: TokenKind::EOF,
                span: Default::default(),
            },
            peek_token: Token { 
                kind: TokenKind::EOF,
                span: Default::default(),
            },
            index: 0,
        };

        parser
    }

    pub fn look(&mut self) -> Option<&char> {
        self.peek()
    }

    // consume
    pub fn eat(&mut self) {
        self.consume_char();
    }

    pub fn current_token(&self) -> &Token {
        &self.current_token
    }

    pub fn advance(&mut self) {
        self.index += 1;
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().unwrap_or(Token { 
            kind: TokenKind::EOF,
            span: Default::default(),
        });
    }

    // the start of the parser
    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.current_token.kind != TokenKind::EOF {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.advance();
        }

        statements
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        // Some(Statement::Let(String::from("sss"), Expression::IntegerLiteral(15)))
    }

    pub fn parse_if_statement(&mut self) -> Option<Statement> {
        Some(Statement::Let(String::from("sss"), Expression::IntegerLiteral(15)))
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.kind {
            TokenKind::Ret => self.parse_return_statement(),
            TokenKind::If => self.parse_if_statement(),
            _ => Some(
                Statement::Return { 
                    value: Some(
                        Box::new(
                            Expression::Unary { 
                                operator: PrefixOp::Not, 
                                operand: Box::new(
                                    Expression::Variable { name: "x".to_string(), span: Span::default() }
                                ), 
                                span: Span::default()
                            }
                        )
                    ), 
                    span: Span::default()
                }
            )
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        
    }
}