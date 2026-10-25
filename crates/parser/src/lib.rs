mod error;

use ast::{Expr, Program, Stmt};
use error::ParserError;
use lexer::Token;
use std::{iter::Peekable, vec::IntoIter};

pub struct Parser {
    pub(crate) current_line: u16,
    pub(crate) tokens: Peekable<IntoIter<Token>>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current_line: 0,
            tokens: vec![].into_iter().peekable(),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Program, ParserError> {
        self.tokens = tokens.into_iter().peekable();
        let mut prog = Program { body: vec![] };

        while let Some(tok) = self.peek() {
            if tok == &Token::EOF {
                break;
            }

            if tok == &Token::NewLine {
                self.current_line += 1;
                self.eat();
                continue;
            }

            prog.body.push(self.parse_stmt()?);
        }

        Ok(prog)
    }

    fn eat(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn expect<F: FnOnce(&Token) -> bool>(&mut self, predicate: F, expected: &str) -> Token {
        use std::process;
        let tk = self.eat().unwrap();
        if predicate(&tk) {
            tk
        } else {
            eprintln!(
                "[line {}] expected a {}, but got {:?}",
                self.current_line, expected, tk
            );
            process::exit(1);
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParserError> {
        let tk = self.peek().unwrap();
        match tk {
            _ => Ok(Stmt::Expression(self.parse_expr()?)),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ParserError> {
        let tk = self.eat().unwrap();

        match tk {
            Token::Identifier(ident) => Ok(Expr::Identifier(ident)),
            Token::Number(num) => Ok(Expr::Number(num.parse::<f64>().unwrap())),
            Token::LeftParen => {
                let value = self.parse_expr();
                self.expect(|tok| tok == &Token::RightParen, "a closing paren");
                value
            }

            _ => Err(ParserError::UnknownToken(self.current_line, tk)),
        }
    }
}
