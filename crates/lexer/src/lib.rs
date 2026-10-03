#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),
    Equals,
    Let,
    OpenParen,
    CloseParen,
    BinaryOperator(char),
    EOF,
    Null,
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut src = source_code.chars().peekable();

    while let Some(tok) = src.next() {
        let token = match tok {
            c if c.is_whitespace() => continue,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' | '-' | '*' | '/' | '%' => Token::BinaryOperator(tok),
            '=' => Token::Equals,
            c if c.is_ascii_digit() => {
                let mut number = String::from(c);

                while let Some(&next) = src.peek() {
                    if next.is_ascii_digit() {
                        number.push(src.next().unwrap());
                    } else {
                        break;
                    }
                }

                Token::Number(number)
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut identifier = String::from(c);

                while let Some(&next) = src.peek() {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        identifier.push(src.next().unwrap());
                    } else {
                        break;
                    }
                }

                match identifier.as_str() {
                    "let" => Token::Let,
                    "null" => Token::Null,
                    _ => Token::Identifier(identifier),
                }
            }
            _ => panic!("Unrecognized character in source code: {}", tok),
        };

        tokens.push(token);
    }

    tokens.push(Token::EOF);

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation() {
        let expected = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::Number("4".to_string()),
            Token::EOF,
        ];
        let tokens = tokenize("let x = 4");

        assert_eq!(tokens, expected);
    }

    #[test]
    fn complex_token_generation() {
        let expected = vec![
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Equals,
            Token::Number("45".to_string()),
            Token::BinaryOperator('*'),
            Token::OpenParen,
            Token::Number("4".to_string()),
            Token::BinaryOperator('/'),
            Token::Number("3".to_string()),
            Token::CloseParen,
            Token::EOF,
        ];
        let tokens = tokenize("let y = 45 * (4/3)");

        assert_eq!(expected, tokens);
    }

    #[test]
    fn eof_token() {
        let expected = vec![Token::EOF];
        let tokens = tokenize("");

        assert_eq!(tokens, expected);
    }
}
