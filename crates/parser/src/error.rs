use lexer::Token;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ParserError {
    UnknownToken(u16, Token),
    ExpectError(u16, String, Token),
    InvalidNumber(u16, String),
    ExpectedToken(u16, String, Token),
    UnexpectedEof(u16),
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
            InvalidNumber(line, n) => {
                write!(f, "[line {}] expected a number but found {:?}", line, n)
            }
            ExpectedToken(line, n, t) => {
                write!(f, "[line {}] expected {} but found {:?}", line, n, t)
            }
            UnexpectedEof(line) => {
                write!(f, "[line {}] unexpected end of file", line)
            }
        }
    }
}
