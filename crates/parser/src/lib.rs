use ast::{Expr, Program, Stmt};
use interpreter::{Token, tokenize};
use std::{iter::Peekable, vec::IntoIter};

pub struct Parser {
    pub tokens: Peekable<IntoIter<Token>>,
}

type Error = Box<dyn std::error::Error>; // For now, till I know all the possible errors

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: vec![].into_iter().peekable(),
        }
    }

    pub fn produce_ast(&mut self, src: &str) -> Result<Program, Error> {
        self.tokens = tokenize(src)?.into_iter().peekable();
        let mut program = Program { statements: vec![] };

        while let Some(token) = self.tokens.peek() {
            if token != &Token::EOF {
                program.statements.push(self.parse_stmt());
            } else {
                break;
            }
        }

        Ok(program)
    }

    fn parse_stmt(&mut self) -> Stmt {
        Stmt::Expr(self.parse_expr())
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let tk = self.tokens.next().unwrap();

        match tk {
            Token::Identifier(ident) => Expr::Identifier(ident),
            Token::Number(num) => {
                if num.contains(".") {
                    Expr::FloatingNumber(num.parse::<f32>().unwrap())
                } else {
                    Expr::Integer(num.parse::<i32>().unwrap())
                }
            }

            _ => {
                eprintln!("Unexpected token found during parsing: {:?}", tk);
                unimplemented!()
            }
        }
    }
}
