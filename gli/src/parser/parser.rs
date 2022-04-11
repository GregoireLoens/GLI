use crate::lexer::lexer::Token;

enum ParseErrorType {
    InvalidToken,
    InvalidExpr,

}

pub struct ParseError {
    error_type: ParseErrorType,
    reason: String
}

pub fn parse(token: &Vec<Token>) -> Result<bool, ParseError> {
    println!("parsing");
    return Ok(true);
} 