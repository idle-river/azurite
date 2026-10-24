use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum LexerError {
    UnknownCharacter(u16, char),
}

impl Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LexerError::*;
        match self {
            UnknownCharacter(line, c) => {
                write!(f, "[line {}] unknown character found: {}", line, c)
            }
        }
    }
}
