use std::env;
use std::fs;
use crate::lexer::lexer::{Token, lex};

pub mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file");
    let mut tokens: Vec<Token> = Vec::new(); 
    for expr in contents.split(";") {
        tokens.append(&mut lex(expr));
    }
    println!("{:?}", tokens);
}