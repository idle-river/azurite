pub enum Token {
    LeftParen,
    RightParen,
    EOF,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::EOF => write!(f, "EOF"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
        }
    }
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut src = src.chars().peekable();

    tokens
}
