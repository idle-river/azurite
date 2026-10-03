#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    Indentifier(String),
    Equals,
    Let,
    OpenParen,
    CloseParen,
    BinaryOperator(char),
    EOF,
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut src = source_code.chars().peekable();

    while let Some(tok) = src.next() {
        let token = match tok {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' | '-' | '*' | '/' | '%' => Token::BinaryOperator(tok),
            '=' => Token::Equals,
            _ => {
                if tok.is_ascii_digit() {
                    let mut num = String::new();

                    while let Some(&next_num) = src.peek() {
                        if next_num.is_ascii_digit() {
                            num.push(src.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    Token::Number(num)
                } else if tok.is_alphabetic() {
                    let mut ident = String::new();

                    while let Some(&next_letter) = src.peek() {
                        if next_letter.is_ascii_digit() {
                            ident.push(src.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    match ident.as_ref() {
                        "let" => Token::Let,
                        _ => Token::Indentifier(ident),
                    }
                } else {
                    panic!("unrecognized character in source code: {}", tok);
                }
            }
        };

        tokens.push(token);
    }

    tokens.push(Token::EOF);

    tokens
}
