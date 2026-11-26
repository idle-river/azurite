#[derive(Debug, PartialEq)]
pub enum Token {
    CompOp(ComparisonOperator),
    BinaryOperator(char),
    Identifier(String),
    Number(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Equals,
    Period,
    Comma,
    EOF,
    Let,
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
    Equal,
    NotEqual,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::EOF => write!(f, "EOF"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
            Token::Period => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::BinaryOperator(op) => write!(f, "BinaryOperator({op})"),
            Token::Identifier(ident) => write!(f, "Identifier({ident})"),
            Token::Number(num) => write!(f, "Number({num})"),
            Token::Let => write!(f, "let"),
            Token::Equals => write!(f, "="),
            Token::CompOp(op) => match op {
                ComparisonOperator::LessThan => write!(f, "<"),
                ComparisonOperator::GreaterThan => write!(f, ">"),
                ComparisonOperator::LessThanEq => write!(f, "<="),
                ComparisonOperator::GreaterThanEq => write!(f, ">="),
                ComparisonOperator::Equal => write!(f, "=="),
                ComparisonOperator::NotEqual => write!(f, "!="),
            },
        }
    }
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut src = src.chars().peekable();
    let mut current_line = 1;

    while let Some(tok) = src.next() {
        let token = match tok {
            // Comments
            '#' => {
                while let Some(&next) = src.peek() {
                    if next == '\n' {
                        break;
                    }
                    src.next();
                }
                current_line += 1;
                continue;
            }

            // Single Character Tokens
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ';' => Token::Semicolon,
            '.' => Token::Period,
            ',' => Token::Comma,
            '+' | '-' | '*' | '/' => Token::BinaryOperator(tok),

            // Two Character Tokens
            '=' => {
                if Some(&'=') == src.peek() {
                    src.next();
                    Token::CompOp(ComparisonOperator::Equal)
                } else {
                    Token::Equals
                }
            }
            '<' => {
                if Some(&'=') == src.peek() {
                    src.next();
                    Token::CompOp(ComparisonOperator::LessThanEq)
                } else {
                    Token::BinaryOperator('<')
                }
            }
            '>' => {
                if Some(&'=') == src.peek() {
                    src.next();
                    Token::CompOp(ComparisonOperator::LessThanEq)
                } else {
                    Token::BinaryOperator('<')
                }
            }

            // Multi Character Tokens
            c if c.is_ascii_digit() => {
                let mut num = c.to_string();
                while let Some(&next) = src.peek() {
                    if next.is_ascii_digit() {
                        num.push(src.next().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Number(num)
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = c.to_string();
                while let Some(&next) = src.peek() {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        ident.push(src.next().unwrap());
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "let" => Token::Let,
                    _ => Token::Identifier(ident),
                }
            }
            c if c.is_whitespace() => {
                if c == '\n' {
                    current_line += 1;
                }
                continue;
            }
            _ => {
                eprintln!(
                    "[line {}] found unexpected character: {}",
                    current_line, tok
                );
                continue;
            }
        };
        tokens.push(token);
    }

    tokens.push(Token::EOF);

    tokens
}
