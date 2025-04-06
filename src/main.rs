
mod lexer;
mod parser;
mod ast;
// mod codegen;
// mod vm;
mod visitor;
mod evaluator;

use std::{cell::RefCell, f32::consts::E, ops::{Deref, DerefMut}, rc::Rc};

use lexer::{Lexer, TokenKind};


// test for lexer
enum _Errors {
    
}

struct Text {
    value: String,
    age: i32
}

impl Text {
    fn _new(value: String) -> Self {
        Self {
            value,
            age: 0
        }
    }

    fn _test(&self) {
        println!("{}", self.value);
        let a = self.age;


    }

    fn _test_mut(&mut self) {
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

// 自动解引用只能用于方法调用 不能用于字段访问
struct Person {
    name: String
}

impl Person {
    fn work(&self) {
        println!("working");
    }

    fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

struct Employee {
    person: Person,
}

impl Deref for Employee {
    type Target = Person;

    fn deref(&self) -> &Self::Target {
        &self.person
    }
}

impl DerefMut for Employee {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.person
    }
}

struct BoxedPerson(Employee);

impl Deref for BoxedPerson {
    type Target = Employee;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxedPerson {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


fn main() {
    // tokenize("x = 2;");
    // let a = '2'.is_alphabetic();
    // println!("{}", a);

    // let a = Box::new(BoxedPerson(Employee { person: Person {} }));
    // a.work();
}
