use crate::error::LexerError;

mod error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,

    // Literals
    Number(String),
    Identifier(String),

    // Symbols
    Equals,
    BinaryOp(char),
    LeftParen,
    RightParen,
    Semicolon,

    // Other
    EOF,
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let mut current_line = 0u16;
    let mut src = src.chars().peekable();

    while let Some(tok) = src.next() {
        let token = match tok {
            // Basic Pattern Matching
            '=' => Token::Equals,
            '+' | '-' | '/' | '*' | '%' | '^' => Token::BinaryOp(tok),
            '(' => Token::LeftParen,
            ')' => Token::LeftParen,
            ';' => Token::Semicolon,

            // Functional Matching
            c if c.is_numeric() => {
                let mut num = String::new();
                num.push(c);

                while let Some(n) = src.peek() {
                    if n.is_numeric() {
                        num.push(src.next().unwrap());
                    } else {
                        break;
                    }
                }

                Token::Number(num)
            }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                ident.push(c);

                while let Some(i) = src.peek() {
                    if i.is_alphabetic() {
                        ident.push(src.next().unwrap());
                    } else {
                        break;
                    }
                }

                match ident.as_ref() {
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
                return Err(LexerError::UnknownCharacter(current_line, tok));
            }
        };

        tokens.push(token);
    }

    tokens.push(Token::EOF);

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let tokens = tokenize("").unwrap();

        assert_eq!(tokens, vec![Token::EOF]);
    }

    #[test]
    fn declaring_ident() {
        let tokens = tokenize("let x = 45;").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Equals,
                Token::Number("45".to_string()),
                Token::Semicolon,
                Token::EOF
            ]
        );
    }
}
