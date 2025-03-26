use std::{iter::Peekable, str::Chars};
use std::str::CharIndices;

pub enum TokenKind {
    // 
    Integer(i16),


    // punctuation
    Plus,          // +
    Minus,         // -
    Asterisk,      // *
    Slash,         // /
    Percent,       // %

    LParen,        // (
    RParen,        // )
    LBrace,        // {
    RBrace,        // }
    LBracket,      // [
    RBracket,      // ]
    Semicolon,     // ;
    Colon,         // :
    Comma,         // ,

    // need judge
    Equal,         // =
    EqualEqual,    // ==
    Bang,          // ！
    BangEqual,     // ！=
    Less,          // <
    LessEqual,     // <=
    Greater,       // > 
    GreaterEqual,  // >=

    // need peek two char
    And,           // &&
    Or,            // ||

    // end
    EOF,
}

// locate the position of the token
// highlights and tips in lsp
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    // caculate the length of the span
    pub fn length(&self) -> usize {
        self.end - self.start
    }

    // judge if the position is in the span
    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }
    
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Token {
//     // keywords
//     Let,
//     Fn,
//     If,
//     Else,
//     While,
//     Return,

//     // 
//     Identifier(String),
//     Integer(i64),
//     String(String),
//     Char(char),
// }

pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self {
            kind,
            span,
        }
    }
}



pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 0,
            read_position: 0,
            ch: '\0',
        }
    }

    // the core of the lexer
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let start_pos = self.position;

        let token = match self.ch {
            // basic char to token
            '+' => Token::new(TokenKind::Plus, self.span()),
            '-' => Token::new(TokenKind::Minus, self.span()),
            '*' => Token::new(TokenKind::Asterisk, self.span()),
            '/' => Token::new(TokenKind::Slash, self.span()),
            '%' => Token::new(TokenKind::Percent, self.span()),

            '(' => Token::new(TokenKind::LParen, self.span()),
            ')' => Token::new(TokenKind::RParen, self.span()),
            '{' => Token::new(TokenKind::LBrace, self.span()),
            '}' => Token::new(TokenKind::RBrace, self.span()),
            '[' => Token::new(TokenKind::LBracket, self.span()),
            ']' => Token::new(TokenKind::RBracket, self.span()),
            ';' => Token::new(TokenKind::Semicolon, self.span()),
            ':' => Token::new(TokenKind::Colon, self.span()),
            ',' => Token::new(TokenKind::Comma, self.span()),

            // need judge
            '=' => {
                if let Some(&'=') = self.peek() {
                    self.read_char();
                    Token::new(TokenKind::EqualEqual, self.span())
                } else {
                    Token::new(TokenKind::Equal, self.span())
                }
            },// ! = how to detect the error and report it? and the way lsp works
            '!' => {
                if let Some(&'=') = self.peek() {
                    self.read_char();
                    Token::new(TokenKind::BangEqual, self.span())
                } else {
                    Token::new(TokenKind::Bang, self.span())
                    
                }
            },
            '<' => {
                if let Some(&'=') = self.peek() {
                    self.read_char();
                    Token::new(TokenKind::LessEqual, self.span())
                } else {
                    Token::new(TokenKind::Less, self.span())
                }
            },
            '>' => {
                if let Some(&'=') = self.peek() {
                    self.read_char();
                    Token::new(TokenKind::GreaterEqual, self.span())
                } else {
                    Token::new(TokenKind::Greater, self.span())
                }
            },

            '&' => {
                self.read_char();
                Token::new(TokenKind::And, self.span())
            },
            '|' => {
                self.read_char();
                Token::new(TokenKind::Or, self.span())
            },

            _ => Token::new(TokenKind::EOF, self.span()),
        };

        // generate the token, determine the exact start and end position of the token
        
        Some(token)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    // consume the char
    fn read_char(&mut self) {
        self.ch = self.input.next().unwrap_or('\0');

        self.position = self.read_position; 
        self.read_position += 1;
    }

    fn read_identifier() {
        todo!()
    }

    fn read_number() {
        todo!()
    }

    // bug here. it can not be give value
    fn span(&self) -> TextSpan {
        TextSpan::new(self.position, self.read_position, self.ch.to_string())
    }

    fn is_at_end(&mut self) -> bool {
        // self.read_position >= self.input.clone().count()
        self.input.peek().is_none()
    }

    // used to judge which function assign to
    fn is_num_start(ch: &char) -> bool {
        ch.is_digit(10)
    }

    fn is_identifier_start(ch: &char) -> bool {
        ch.is_alphabetic() || *ch == '_'
    }


    
}