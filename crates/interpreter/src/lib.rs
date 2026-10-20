#[derive(Debug, Clone)]
pub enum Token {
    // Literal
    Identifier(String),
    Number(String),
    BinaryOperator(char),
    Null,

    // Other
    OpenParen,
    CloseParen,
    Equals,
    Semicolon,
    Let,
    EOF,
}

macro_rules! interpreter_error {
    ($line :expr, $where: expr, $message: expr) => {{
        println!("[line {}] Error {}: {}", $line, $where, $message);
        ::std::process::exit(1);
    }};
    ($line:expr,$message:expr) => {{
        println!("[line {}] Error: {}", $line, $message);
        ::std::process::exit(1);
    }};
}

pub fn tokenize(src: String) -> Vec<Token> {
    let mut current_lines = 1u16;

    let mut tokens = Vec::<Token>::new();
    let mut chars = src.chars().peekable();

    while let Some(c) = chars.next() {
        let new_token = match c {
            // Basic Pattern Matching
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' | '-' | '/' | '*' | '%' => Token::BinaryOperator(c),
            ';' => Token::Semicolon,
            '=' => Token::Equals,

            // Literals
            c if c.is_whitespace() => {
                if c == '\n' {
                    current_lines += 1;
                }

                continue;
            }
            c if c.is_alphabetic() => {
                let mut ident = String::from(c);

                while let Some(next_char) = chars.peek() {
                    if !next_char.is_alphabetic() {
                        break;
                    }

                    ident.push(chars.next().unwrap());
                }

                match ident.as_ref() {
                    "let" => Token::Let,
                    "null" => Token::Null,
                    _ => Token::Identifier(ident),
                }
            }
            c if c.is_numeric() => {
                let mut num = String::from(c);

                while let Some(next_char) = chars.peek() {
                    if !next_char.is_numeric() {
                        break;
                    }

                    num.push(chars.next().unwrap());
                }

                Token::Number(num)
            }

            // Unknown token
            _ => interpreter_error!(current_lines, format!("Unexpected character - {}", c)),
        };

        tokens.push(new_token);
    }

    tokens.push(Token::EOF);

    tokens
}
