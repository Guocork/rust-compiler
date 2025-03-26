use crate::{ast::{Expression, Statement}, lexer::{Lexer, Token}};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
            peek_token: Token::EOF
        };

        parser
    }

    pub fn parse_program(&mut self) {
        
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        Some(Statement::Let(String::from("sss"), Expression::IntegerLiteral(15)))
    }

    pub fn parse_if_statement(&mut self) -> Option<Statement> {
        Some(Statement::Let(String::from("sss"), Expression::IntegerLiteral(15)))
    }

    pub fn parse_while_statement(&mut self) -> Option<Statement> {
        Some(Statement::Let(String::from("sss"), Expression::IntegerLiteral(15)))
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_statement(),
            Token::Return => self.parse_return_statement(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            _ => Some(Statement::Let(String::from("sss"), Expression::IntegerLiteral(15)))
        }
    }

    // fn parse_expression(&mut self) -> Option<Expression> {
        
    // }
}