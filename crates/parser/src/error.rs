use lexer::Token;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ParserError {
    UnknownToken(u16, Token),
    ExpectError(u16, String, Token),
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParserError::*;
        match self {
            UnknownToken(line, t) => {
                write!(f, "[line {}] unknown token found: {:?}", line, t)
            }
            ExpectError(line, a, t) => {
                write!(f, "[line {}] expected {} but found {:?}", line, a, t)
            }
        }
    }
}
