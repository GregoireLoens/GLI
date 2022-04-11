#[derive(Debug)]
enum TokenType {
    Num,
    Plus,
    Times,
    Minus,
    Identifier,
    True,
    False,
    Eof,
    Unknown,
} 

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    token_text: String
}


pub fn lex(exp: &str) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    for elem in exp.split(" ") {
        if elem.to_string().parse::<f64>().is_ok() {
            tokens.push(Token {
                token_type: TokenType::Num,
                token_text: elem.to_string()
            });
        }
        else {
            match elem {
                "+" => tokens.push(Token {
                    token_type: TokenType::Plus,
                    token_text: elem.to_string()                
                }),
                "*" => tokens.push(Token {
                    token_type: TokenType::Times,
                    token_text: elem.to_string()                
                }),
                ""
                _ => tokens.push(Token {
                    token_type: TokenType::Eof,
                    token_text: elem.to_string()                
                }),
            }
        }
    }
    return tokens;
}