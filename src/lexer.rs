use std::{fmt::Display, iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // value
    Integer(i16),  // 1 2 3 .....
    // String(String),// "hello world"
    // Char(char),    // 'a' 'b' 'c' .....
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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Integer(value) => write!(f, "Integer({})", value),
            TokenKind::Identifier(value) => write!(f, "Identifier({})", value),
            TokenKind::Def => write!(f, "Def"),
            TokenKind::Fun => write!(f, "Fun"),
            TokenKind::Ret => write!(f, "Ret"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::For => write!(f, "For"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Asterisk => write!(f, "Asterisk"),
            TokenKind::Slash => write!(f, "Slash"),
            TokenKind::Percent => write!(f, "Percent"), 
            TokenKind::LParen => write!(f, "LParen"),
            TokenKind::RParen => write!(f, "RParen"),
            TokenKind::LBrace => write!(f, "LBrace"),
            TokenKind::RBrace => write!(f, "RBrace"),
            TokenKind::LBracket => write!(f, "LBracket"),
            TokenKind::RBracket => write!(f, "RBracket"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Equal => write!(f, "Equal"),
            TokenKind::EqualEqual => write!(f, "EqualEqual"),
            TokenKind::Bang => write!(f, "Bang"),
            TokenKind::BangEqual => write!(f, "BangEqual"),
            TokenKind::Less => write!(f, "Less"),
            TokenKind::LessEqual => write!(f, "LessEqual"),
            TokenKind::Greater => write!(f, "Greater"),
            TokenKind::GreaterEqual => write!(f, "GreaterEqual"),
            TokenKind::And => write!(f, "And"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::EOF => write!(f, "EOF"),
        }
    }
}

// locate the position of the token
// highlights and tips in lsp
#[derive(Debug, Clone, PartialEq, Default)]
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
    pub fn _length(&self) -> usize {
        self.end - self.start
    }

    // judge if the position is in the span
    pub fn _contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }
    
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
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
        let orinial_str = self.input.clone().collect::<String>();
        self.consume_char();


        let start_pos = self.position;

        if Self::is_num_start(&self.ch) {
            let num = self.read_number();
            let token_kind = Self::handle_number(num.clone());
            return Some(Token::new(token_kind, TextSpan::new(start_pos, self.position, num.to_string())));
        } else if Self::is_identifier_start(&self.ch) {
            let iden = self.read_identifier();
            let token_kind = Self::handle_identifier(iden.clone());
            return Some(Token::new(token_kind, TextSpan::new(start_pos, self.position, iden)));
        }

        let token_kind = match self.ch {
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
        };

        // generate the token, determine the exact start and end position of the token
        let end_pos = self.position;
        let _a = orinial_str
            .chars()
            .enumerate()
            .filter(|(i,_)| *i >= start_pos && *i <= end_pos)
            .map(|(_, ch)| ch)
            .collect::<String>();
        let span = TextSpan::new(start_pos, end_pos, self.ch.to_string());
        let token = Token::new(token_kind, span);
        Some(token)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    // consume the char
    pub fn consume_char(&mut self) {
        self.ch = self.input.next().unwrap_or('\0');

        self.position = self.read_position; 
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let mut token_value = String::from(self.ch);
        while let Some(&ch) = self.input.peek() {
            if Self::is_identifier_start(&ch) {
                token_value.push(ch);
                self.consume_char();
            } else {
                break;
            }
        }

        token_value
    }

    fn handle_identifier(iden: String) -> TokenKind {
        match iden.as_str() {
            "def" => TokenKind::Def,
            "fun" => TokenKind::Fun,
            "ret" => TokenKind::Ret,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            _ => TokenKind::Identifier(iden),
        }
    }
    

    fn read_number(&mut self) -> String {
        let mut token_value = String::from(self.ch);
        while let Some(&ch) = self.input.peek() {
            if ch.is_digit(10) {
                token_value.push(ch);
                self.consume_char();
            } else {
                break;
            }
        }
        token_value
    }

    fn handle_number(num: String) -> TokenKind {
        let value = num.parse::<i16>().unwrap();
        TokenKind::Integer(value)
    }

    fn _is_at_end(&mut self) -> bool {
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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer1() {
        let input = "def fun ret if else for + - * / % ( ) { } [ ] ; : , = == ! != < <= > >= && ||";
        let mut lexer = Lexer::new(input);
        while let Some(token) = lexer.next_token() {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_lexer2() {
        let input = "
        def a = 5;
        def x = 5;
        def y = 10;
        if (x < y) {
            ret true;
        } else {
            ret false;
        }
    ";

        let mut lexer = Lexer::new(input);
        loop {
            let token = lexer.next_token().unwrap();
            println!("{:?}", token);
            if token.kind == TokenKind::EOF {
                break;
            }
        }
    }
}