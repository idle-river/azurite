use std::collections::VecDeque;

use ast::{Expr, Program, Stmt};
use lexer::{Token, tokenize};

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            tokens: VecDeque::new(),
        }
    }

    pub fn produce_ast(&mut self, source_code: &str) -> Program {
        let mut program = Program::new();

        self.tokens = VecDeque::from(tokenize(source_code));

        // Parse til end of file
        while self.not_eof() {
            program.body.push(self.parse_stmt());
        }

        program
    }

    fn not_eof(&self) -> bool {
        self.tokens[0] != Token::EOF
    }

    fn eat(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }

    fn parse_stmt(&mut self) -> Stmt {
        // no other stmts beside an expr
        Stmt::Expression(self.parse_expr())
    }

    fn parse_expr(&mut self) -> Expr {
        // just implementing primary exprs first
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let token = self.eat();

        match token {
            Token::Identifier(ident) => Expr::Identifier(ident),
            Token::Number(num) => Expr::NumericLiteral(num.parse().unwrap()),
            _ => panic!("Unexpected token found during parsing: {:?}", token),
        }
    }
}
