#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    Indentifier(String),
    Equals,
    OpenParen,
    CloseParen,
    BinaryOperator(char),
    EOF,
}

pub fn tokenize(sourceCode: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    tokens
}
