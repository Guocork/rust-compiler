use std::{iter::Peekable, str::Chars};
use std::str::CharIndices;

pub enum TokenKind {
    // value
    Integer(i16),  // 1 2 3 .....
    String(String),// "hello world"
    Char(char),    // 'a' 'b' 'c' .....
    Identifier(String), // variable name

    // keuwords
    Def,           // def
    Fun,           // fun
    Ret,           // ret --> return
    If,            // if
    Else,          // else
    For,           // for

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

        let token_kind = if Self::is_num_start(&self.ch) {
            self.handle_number()
        } else if Self::is_identifier_start(&self.ch) {
            self.handle_identifier()
        } else {
            match self.ch {
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Asterisk,
                '/' => TokenKind::Slash,
                '%' => TokenKind::Percent,
    
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '{' => TokenKind::LBrace,
                '}' => TokenKind::RBrace,
                '[' => TokenKind::LBracket,
                ']' => TokenKind::RBracket,
                ';' => TokenKind::Semicolon,
                ':' => TokenKind::Colon,
                ',' => TokenKind::Comma,
    
                '=' => {
                    self.handle_double_char('=', TokenKind::Equal, TokenKind::EqualEqual)
                },// ! = how to detect the error and report it? and the way lsp works
                '!' => {
                    self.handle_double_char('=', TokenKind::Bang, TokenKind::BangEqual)
                },
                '<' => {
                    self.handle_double_char('=', TokenKind::Less, TokenKind::LessEqual)
                },
                '>' => {
                    self.handle_double_char('=', TokenKind::Greater, TokenKind::GreaterEqual)
                },
    
                '&' => {
                    self.consume_char();
                    TokenKind::And
                },
                '|' => {
                    self.consume_char();
                    TokenKind::Or
                },
    
                _ => TokenKind::EOF,
            }
        };

        // 复习一下 索引 字符 和 字节 
        // generate the token, determine the exact start and end position of the token
        let end_pos = self.position;
        let a = self.input
            .clone()
            .enumerate()
            .filter(|(i,_)| *i >= start_pos && *i < end_pos)
            .map(|(_, ch)| ch)
            .collect::<String>();
        let span = TextSpan::new(start_pos, end_pos, a);
        let token = Token::new(token_kind, span);
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
    fn consume_char(&mut self) {
        self.ch = self.input.next().unwrap_or('\0');

        self.position = self.read_position; 
        self.read_position += 1;
    }

    fn handle_identifier(&mut self) -> TokenKind {
        let mut token_value = String::from(self.ch);
        while let Some(&ch) = self.input.peek() {
            if Self::is_identifier_start(&ch) {
                token_value.push(ch);
                self.consume_char();
            } else {
                break;
            }
        }
        match token_value.as_str() {
            "def" => TokenKind::Def,
            "fun" => TokenKind::Fun,
            "ret" => TokenKind::Ret,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            _ => TokenKind::Identifier(token_value),
        }
    }

    fn handle_number(&mut self) -> TokenKind {
        let mut token_value = String::from(self.ch);
        while let Some(&ch) = self.input.peek() {
            if ch.is_digit(10) {
                token_value.push(ch);
                self.consume_char();
            } else {
                break;
            }
        }
        let value = token_value.parse::<i16>().unwrap();
        TokenKind::Integer(value)
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

    fn handle_double_char(&mut self, pun: char, single: TokenKind, double: TokenKind) -> TokenKind {
        if let Some(&ch) = self.peek() {
            if ch == pun {
                self.consume_char();
                double
            } else {
                single
            }
        } else {
            single
        }
        
    }
}