mod error;

use ast::{BinaryOp, Expr, Program, Stmt};
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

    fn expect<F>(&mut self, predicate: F, expected: &str) -> Result<Token, ParserError>
    where
        F: FnOnce(&Token) -> bool,
    {
        // Getting it rn so that we don't borrow twice inside the match
        let current_line = self.current_line;

        match self.peek() {
            Some(tk) if predicate(tk) => Ok(self.eat().unwrap()),
            Some(tk) => Err(ParserError::ExpectedToken(
                current_line,
                expected.to_string(),
                tk.clone(),
            )),
            None => Err(ParserError::UnexpectedEof(current_line)),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParserError> {
        Ok(Stmt::Expression(self.parse_expr()?))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_multiplicitive_expr()?;

        while let Some(Token::BinaryOp(op)) = self.peek() {
            if *op != '+' && *op != '-' {
                break;
            }

            let Token::BinaryOp(op) = self.eat().unwrap() else {
                unreachable!()
            };

            let right = self.parse_multiplicitive_expr()?;

            left = Expr::Binary {
                op: BinaryOp::new(op),
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicitive_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_primary_expr()?;

        while let Some(Token::BinaryOp(op)) = self.peek() {
            if *op != '*' && *op != '/' && *op != '%' {
                break;
            }

            let Token::BinaryOp(op) = self.eat().unwrap() else {
                unreachable!()
            };

            let right = self.parse_primary_expr()?;

            left = Expr::Binary {
                op: BinaryOp::new(op),
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ParserError> {
        let tk = self
            .eat()
            .ok_or(ParserError::UnexpectedEof(self.current_line))?;

        match tk {
            Token::Identifier(ident) => Ok(Expr::Identifier(ident)),
            Token::Number(num) => {
                let value = num
                    .parse::<f64>()
                    .map_err(|_| ParserError::InvalidNumber(self.current_line, num.clone()))?;

                Ok(Expr::Number(value))
            }
            Token::LeftParen => {
                let value = self.parse_expr()?;
                self.expect(|tok| tok == &Token::RightParen, "a right paren")?;

                Ok(value)
            }

            _ => Err(ParserError::UnknownToken(self.current_line, tk)),
        }
    }
}
