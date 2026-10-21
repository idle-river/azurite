#[derive(Debug, Clone)]
pub enum Token {
    // Literal
    Identifier(String),
    Number(String),
    BinaryOperator(char),
    ComparisonOperator(CompOperator), // Could Be ==, >=, <=, !, etc...
    StrLiteral(String),
    Null,

    // Other
    OpenParen,
    CloseParen,
    Equals,
    Semicolon,
    Let,
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub enum CompOperator {
    Bang, // !
    BangBang,
    EqualsEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    NotEqual,
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
            // Handling Comments
            '#' => {
                for next_char in chars.by_ref() {
                    if next_char == '\n' {
                        break;
                    }
                }

                continue;
            }

            // Basic Pattern Matching
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' | '-' | '/' | '*' | '%' => Token::BinaryOperator(c),
            ';' => Token::Semicolon,
            '!' | '=' | '>' | '<' => {
                let mut op = String::from(c);

                if let Some(&next_char) = chars.peek()
                    && next_char == '='
                {
                    op.push(chars.next().unwrap());
                }

                if op == "=" {
                    Token::Equals
                } else {
                    Token::ComparisonOperator(match op.as_str() {
                        "!" => CompOperator::Bang,
                        "!=" => CompOperator::NotEqual,
                        "==" => CompOperator::EqualsEquals,
                        ">" => CompOperator::Greater,
                        ">=" => CompOperator::GreaterEquals,
                        "<" => CompOperator::Less,
                        "<=" => CompOperator::LessEquals,
                        _ => unreachable!(),
                    })
                }
            }

            '"' => {
                let mut string = String::new();

                loop {
                    match chars.next() {
                        Some('"') => break, // Closing quote found
                        Some(c) => {
                            if c == '\n' {
                                interpreter_error!(
                                    current_lines,
                                    "string does not end on the same line"
                                );
                            } else {
                                string.push(c);
                            }
                        }
                        None => {
                            // Unterminated string literal!
                            interpreter_error!(current_lines, "Unterminated string literal");
                        }
                    }
                }

                Token::StrLiteral(string)
            }

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
