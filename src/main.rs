
mod lexer;
mod parser;
mod ast;
mod codegen;
mod vm;


// test for lexer
enum Errors {
    
}

#[derive(Debug, Clone)]
enum TokenType {
    SemoColon,
    Identifier,
    Equal
}


#[derive(Debug, Clone)]
struct Token {
    pub ty: TokenType,
    pub value: String
}


fn tokenize(code: &str) -> Result<Vec<Token>, Errors> {

    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token: Option<Token>  = None;

    for c in code.chars() {
        if c == ';' {
            if let Some(tok) = current_token {
                tokens.push(tok);
            }
            tokens.push(Token {
                ty: TokenType::SemoColon,
                value: String::from(";")
            });

            current_token = None;
        } else if c == '=' {
            if let Some(tok) = current_token {
                tokens.push(tok);
            }

            tokens.push(Token {
                ty: TokenType::Equal,
                value: String::from("=")
            });
            current_token = None;
            
        } else if (c >= 'A' && c <= 'Z') || c == '_' {
            if let Some(mut tok) = current_token {
                tok.value.push(c);
            } else {
                current_token = Some(Token {
                    ty: TokenType::Identifier,
                    value: c.to_string()
                });
            }
        }
    }

    Ok(tokens)
}



fn main() {
    tokenize("x = 2;");
}
