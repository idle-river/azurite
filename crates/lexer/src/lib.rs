use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literal
    Identifier(String),
    Number(String),
    BinaryOperator(char),
    ComparisonOperator(CompOperator), // ==, >=, <=, !, etc...
    StrLiteral(String),

    // Keywords
    Let,
    Const,
    Fn,

    // Syntax
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Dot,
    Comma,
    Colon,
    Equals,
    Semicolon,
    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompOperator {
    Bang, // !
    EqualsEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    NotEqual,
}

/// An error raised while lexing/parsing/interpreting, carrying the line it happened on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterpreterError {
    pub line: usize,
    pub location: Option<String>,
    pub message: String,
}

impl InterpreterError {
    pub fn new(line: usize, location: Option<String>, message: String) -> Self {
        Self {
            line,
            location,
            message,
        }
    }
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.location {
            Some(loc) => write!(f, "[line {}] Error {}: {}", self.line, loc, self.message),
            None => write!(f, "[line {}] Error: {}", self.line, self.message),
        }
    }
}

impl std::error::Error for InterpreterError {}

/// Builds an `InterpreterError`. Use it as `return Err(interpreter_error!(line, "msg"))`.
///
/// Exported at the crate root, so other modules can call `crate::interpreter_error!`.
#[macro_export]
macro_rules! interpreter_error {
    ($line:expr, $where:expr, $message:expr) => {
        $crate::InterpreterError::new(
            $line,
            ::std::option::Option::Some(($where).to_string()),
            ($message).to_string(),
        )
    };
    ($line:expr, $message:expr) => {
        $crate::InterpreterError::new($line, ::std::option::Option::None, ($message).to_string())
    };
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, InterpreterError> {
    let mut line = 1usize;
    let mut tokens = Vec::<Token>::new();
    let mut chars = src.chars().peekable();

    while let Some(c) = chars.next() {
        let new_token = match c {
            // Comments run to end of line; the newline still counts as a line.
            '#' => {
                for next_char in chars.by_ref() {
                    if next_char == '\n' {
                        line += 1;
                        break;
                    }
                }
                continue;
            }

            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' | '-' | '/' | '*' | '%' => Token::BinaryOperator(c),
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            '.' => Token::Dot,

            '!' | '=' | '>' | '<' => {
                let has_eq = chars.next_if_eq(&'=').is_some();
                match (c, has_eq) {
                    ('=', false) => Token::Equals,
                    ('=', true) => Token::ComparisonOperator(CompOperator::EqualsEquals),
                    ('!', false) => Token::ComparisonOperator(CompOperator::Bang),
                    ('!', true) => Token::ComparisonOperator(CompOperator::NotEqual),
                    ('>', false) => Token::ComparisonOperator(CompOperator::Greater),
                    ('>', true) => Token::ComparisonOperator(CompOperator::GreaterEquals),
                    ('<', false) => Token::ComparisonOperator(CompOperator::Less),
                    ('<', true) => Token::ComparisonOperator(CompOperator::LessEquals),
                    _ => unreachable!(),
                }
            }

            '"' => {
                let mut string = String::new();

                loop {
                    match chars.next() {
                        Some('"') => break,
                        Some('\n') => {
                            return Err(interpreter_error!(
                                line,
                                "string does not end on the same line"
                            ));
                        }
                        Some(ch) => string.push(ch),
                        None => {
                            return Err(interpreter_error!(line, "Unterminated string literal"));
                        }
                    }
                }

                Token::StrLiteral(string)
            }

            c if c.is_whitespace() => {
                if c == '\n' {
                    line += 1;
                }
                continue;
            }

            // Identifiers: start with a letter or `_`, continue with letters, digits, or `_`.
            c if c.is_alphabetic() || c == '_' => {
                let mut ident = String::from(c);

                while let Some(next) = chars.next_if(|n| n.is_alphanumeric() || *n == '_') {
                    ident.push(next);
                }

                match ident.as_str() {
                    "let" => Token::Let,
                    "const" => Token::Const,
                    "fn" => Token::Fn,
                    _ => Token::Identifier(ident),
                }
            }

            c if c.is_numeric() => {
                let mut num = String::from(c);

                while let Some(next) = chars.next_if(|n| n.is_numeric() || *n == '.') {
                    num.push(next);
                }

                Token::Number(num)
            }

            _ => {
                return Err(interpreter_error!(
                    line,
                    format!("Unexpected character - {}", c)
                ));
            }
        };

        tokens.push(new_token);
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    fn lex(src: &str) -> Vec<Token> {
        tokenize(src).expect("expected lexing to succeed")
    }

    fn lex_err(src: &str) -> InterpreterError {
        tokenize(src).expect_err("expected lexing to fail")
    }

    fn ident(s: &str) -> Token {
        Identifier(s.into())
    }

    fn num(s: &str) -> Token {
        Number(s.into())
    }

    // ---------- Basics ----------

    #[test]
    fn empty_input_yields_only_eof() {
        assert_eq!(lex(""), vec![EOF]);
    }

    #[test]
    fn whitespace_only_yields_only_eof() {
        assert_eq!(lex("  \t\n\r\n  "), vec![EOF]);
    }

    #[test]
    fn single_char_tokens() {
        assert_eq!(lex("( ) ;"), vec![OpenParen, CloseParen, Semicolon, EOF]);
    }

    // ---------- Operators ----------

    #[test]
    fn all_binary_operators() {
        assert_eq!(
            lex("+ - / * %"),
            vec![
                BinaryOperator('+'),
                BinaryOperator('-'),
                BinaryOperator('/'),
                BinaryOperator('*'),
                BinaryOperator('%'),
                EOF
            ]
        );
    }

    #[test]
    fn negative_number_is_operator_then_number() {
        assert_eq!(lex("-5"), vec![BinaryOperator('-'), num("5"), EOF]);
    }

    #[test]
    fn single_equals_is_assignment() {
        assert_eq!(lex("="), vec![Equals, EOF]);
    }

    #[test]
    fn all_comparison_operators() {
        assert_eq!(
            lex("! != == > >= < <="),
            vec![
                ComparisonOperator(CompOperator::Bang),
                ComparisonOperator(CompOperator::NotEqual),
                ComparisonOperator(CompOperator::EqualsEquals),
                ComparisonOperator(CompOperator::Greater),
                ComparisonOperator(CompOperator::GreaterEquals),
                ComparisonOperator(CompOperator::Less),
                ComparisonOperator(CompOperator::LessEquals),
                EOF
            ]
        );
    }

    #[test]
    fn operators_without_spaces() {
        assert_eq!(
            lex("a>=b"),
            vec![
                ident("a"),
                ComparisonOperator(CompOperator::GreaterEquals),
                ident("b"),
                EOF
            ]
        );
    }

    #[test]
    fn double_bang_is_two_bangs() {
        assert_eq!(
            lex("!!"),
            vec![
                ComparisonOperator(CompOperator::Bang),
                ComparisonOperator(CompOperator::Bang),
                EOF
            ]
        );
    }

    // ---------- Keywords / identifiers ----------

    #[test]
    fn keywords() {
        assert_eq!(
            lex("let null"),
            vec![Let, Identifier("null".to_string()), EOF]
        );
    }

    #[test]
    fn keyword_prefix_is_identifier() {
        assert_eq!(
            lex("letter nullable"),
            vec![ident("letter"), ident("nullable"), EOF]
        );
    }

    #[test]
    fn keywords_are_case_sensitive() {
        assert_eq!(lex("Let NULL"), vec![ident("Let"), ident("NULL"), EOF]);
    }

    #[test]
    fn identifier_with_digits() {
        assert_eq!(lex("x1 abc123"), vec![ident("x1"), ident("abc123"), EOF]);
    }

    #[test]
    fn identifier_with_underscores() {
        assert_eq!(
            lex("snake_case _private __x_1_"),
            vec![ident("snake_case"), ident("_private"), ident("__x_1_"), EOF]
        );
    }

    #[test]
    fn lone_underscore_is_identifier() {
        assert_eq!(lex("_"), vec![ident("_"), EOF]);
    }

    #[test]
    fn keyword_with_underscore_or_digit_is_identifier() {
        assert_eq!(lex("let_ null2"), vec![ident("let_"), ident("null2"), EOF]);
    }

    #[test]
    fn unicode_identifier() {
        assert_eq!(lex("héllo"), vec![ident("héllo"), EOF]);
    }

    // ---------- Numbers ----------

    #[test]
    fn integer_and_float() {
        assert_eq!(lex("42 3.14"), vec![num("42"), num("3.14"), EOF]);
    }

    #[test]
    fn malformed_number_is_accepted_by_lexer() {
        // The lexer doesn't validate; the parser must reject this.
        assert_eq!(lex("1.2.3"), vec![num("1.2.3"), EOF]);
    }

    #[test]
    fn trailing_dot_number() {
        assert_eq!(lex("5."), vec![num("5."), EOF]);
    }

    #[test]
    fn number_directly_followed_by_identifier() {
        // Numbers can't contain letters, so this still splits.
        assert_eq!(lex("5abc"), vec![num("5"), ident("abc"), EOF]);
    }

    // ---------- Strings ----------

    #[test]
    fn simple_string() {
        assert_eq!(lex(r#""hello""#), vec![StrLiteral("hello".into()), EOF]);
    }

    #[test]
    fn empty_string() {
        assert_eq!(lex(r#""""#), vec![StrLiteral("".into()), EOF]);
    }

    #[test]
    fn string_preserves_inner_whitespace_and_symbols() {
        assert_eq!(
            lex(r##""a + b # not a comment""##),
            vec![StrLiteral("a + b # not a comment".into()), EOF]
        );
    }

    #[test]
    fn adjacent_strings() {
        assert_eq!(
            lex(r#""a""b""#),
            vec![StrLiteral("a".into()), StrLiteral("b".into()), EOF]
        );
    }

    // ---------- Comments ----------

    #[test]
    fn comment_only() {
        assert_eq!(lex("# just a comment"), vec![EOF]);
    }

    #[test]
    fn comment_skips_to_end_of_line() {
        assert_eq!(lex("let # ignored ( ) ;\nx"), vec![Let, ident("x"), EOF]);
    }

    #[test]
    fn comment_at_end_of_line_after_code() {
        assert_eq!(lex("x; # trailing"), vec![ident("x"), Semicolon, EOF]);
    }

    // ---------- Full programs ----------

    #[test]
    fn let_statement() {
        assert_eq!(
            lex("let x = 5 + 3;"),
            vec![
                Let,
                ident("x"),
                Equals,
                num("5"),
                BinaryOperator('+'),
                num("3"),
                Semicolon,
                EOF
            ]
        );
    }

    #[test]
    fn multiline_program() {
        let src = "let a = 1;\n# comment\nlet b = \"hi\";\n";
        assert_eq!(
            lex(src),
            vec![
                Let,
                ident("a"),
                Equals,
                num("1"),
                Semicolon,
                Let,
                ident("b"),
                Equals,
                StrLiteral("hi".into()),
                Semicolon,
                EOF
            ]
        );
    }

    // ---------- Errors ----------

    #[test]
    fn unterminated_string() {
        let e = lex_err(r#""abc"#);
        assert_eq!(e.line, 1);
        assert_eq!(e.message, "Unterminated string literal");
    }

    #[test]
    fn newline_inside_string() {
        let e = lex_err("\"abc\ndef\"");
        assert_eq!(e.line, 1);
        assert_eq!(e.message, "string does not end on the same line");
    }

    #[test]
    fn unexpected_character() {
        for bad in ["@", "$", "&", "~", "`"] {
            let e = lex_err(bad);
            assert_eq!(e.line, 1, "input: {bad}");
            assert_eq!(e.message, format!("Unexpected character - {bad}"));
        }
    }

    #[test]
    fn error_line_counts_plain_newlines() {
        let e = lex_err("let x = 1;\nlet y = @;");
        assert_eq!(e.line, 2);
    }

    #[test]
    fn error_line_counts_comment_newlines() {
        // Regression: comments used to swallow their '\n' without bumping the line.
        let e = lex_err("# one\n# two\n@");
        assert_eq!(e.line, 3);
    }

    #[test]
    fn error_line_after_trailing_comment_on_code_line() {
        let e = lex_err("let x = 1; # note\n$");
        assert_eq!(e.line, 2);
    }

    #[test]
    fn unterminated_string_line_after_comment() {
        let e = lex_err("# hi\n\"abc");
        assert_eq!(e.line, 2);
        assert_eq!(e.message, "Unterminated string literal");
    }

    #[test]
    fn newline_in_string_line_after_comment_and_blank_lines() {
        let e = lex_err("# hi\n\n\"abc\ndef\"");
        assert_eq!(e.line, 3);
    }

    #[test]
    fn display_without_location() {
        let e = interpreter_error!(7, "boom");
        assert_eq!(e.to_string(), "[line 7] Error: boom");
    }

    #[test]
    fn display_with_location() {
        let e = interpreter_error!(7, "at 'x'", "boom");
        assert_eq!(e.to_string(), "[line 7] Error at 'x': boom");
        assert_eq!(e.location.as_deref(), Some("at 'x'"));
    }
}
