
mod lexer;
// mod parser;
// mod ast;
// mod codegen;
// mod vm;

use lexer::{Lexer, TokenKind};


// test for lexer
enum Errors {
    
}

struct Text {
    value: String,
    age: i32
}

impl Text {
    fn new(value: String) -> Self {
        Self {
            value,
            age: 0
        }
    }

    fn test(&self) {
        println!("{}", self.value);
        let a = self.age;


    }

    fn test_mut(&mut self) {
        self.value = String::from("Hello");
        self.age += 1;
        let age_ref = &mut self.age;
        let b = *age_ref;
        *age_ref += 1;
        
    }
    
}

// #[derive(Debug, Clone)]
// enum TokenType {
//     SemoColon,
//     Identifier,
//     Equal
// }


// #[derive(Debug, Clone)]
// struct Token {
//     pub ty: TokenType,
//     pub value: String
// }


// fn tokenize(code: &str) -> Result<Vec<Token>, Errors> {

//     let mut tokens: Vec<Token> = Vec::new();
//     let mut current_token: Option<Token>  = None;

//     for c in code.chars() {
//         if c == ';' {
//             if let Some(tok) = current_token {
//                 tokens.push(tok);
//             }
//             tokens.push(Token {
//                 ty: TokenType::SemoColon,
//                 value: String::from(";")
//             });

//             current_token = None;
//         } else if c == '=' {
//             if let Some(tok) = current_token {
//                 tokens.push(tok);
//             }

//             tokens.push(Token {
//                 ty: TokenType::Equal,
//                 value: String::from("=")
//             });
//             current_token = None;
            
//         } else if (c >= 'A' && c <= 'Z') || c == '_' {
//             if let Some(mut tok) = current_token {
//                 tok.value.push(c);
//             } else {
//                 current_token = Some(Token {
//                     ty: TokenType::Identifier,
//                     value: c.to_string()
//                 });
//             }
//         }
//     }

//     Ok(tokens)
// }



fn main() {
    // tokenize("x = 2;");
    // let a = '2'.is_alphabetic();
    // println!("{}", a);
    let input = "
        def a = 5;
        let x = 5;
        let y = 10;
        if (x < y) {
            return true;
        } else {
            return false;
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
