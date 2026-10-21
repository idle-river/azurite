use ast::{BinaryOp, Expr, Program, Stmt};
use lexer::{Token, tokenize};
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

    pub fn expect(&mut self, predicate: impl FnOnce(&Token) -> bool) -> Token {
        use std::process::exit;
        match self.tokens.next() {
            Some(tok) if predicate(&tok) => tok,
            Some(other) => {
                eprintln!("Unexpected token; found {:?}", other);
                exit(1);
            }
            None => {
                eprintln!("Token not found");
                exit(1);
            }
        }
    }

    fn parse_stmt(&mut self) -> Stmt {
        let tk = self.tokens.peek().unwrap();
        match tk {
            Token::Let | Token::Const => self.parse_var_decl(),
            _ => Stmt::Expr(self.parse_expr()),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_additive_expr()
    }

    fn parse_var_decl(&mut self) -> Stmt {
        let is_constant = self.tokens.next().unwrap() == Token::Const;
        let Token::Identifier(ident) = self.expect(|tok| matches!(tok, &Token::Identifier(_)))
        else {
            unreachable!()
        };

        self.expect(|tok| tok == &Token::Equals);

        let val = self.parse_expr();
        self.expect(|tok| tok == &Token::Semicolon);

        Stmt::VarDecl {
            constant: is_constant,
            identifier: ident,
            value: val,
        }
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let tk = self.tokens.next().unwrap();

        match tk {
            Token::Identifier(ident) => Expr::Identifier(ident),
            Token::Number(num) => {
                if num.contains(".") {
                    Expr::FloatingNumber(num.parse::<f64>().unwrap())
                } else {
                    Expr::Integer(num.parse::<i32>().unwrap())
                }
            }
            Token::OpenParen => {
                let expr = self.parse_additive_expr();
                self.expect(|tok| tok == &Token::CloseParen);

                expr
            }

            _ => {
                eprintln!("Unexpected token found during parsing: {:?}", tk);
                unimplemented!()
            }
        }
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let mut left = self.parse_multiplicative_expr();

        while let Some(&Token::BinaryOperator(c)) = self.tokens.peek() {
            if !matches!(c, '+' | '-') {
                break; // not ours, leave it for a caller
            }
            self.tokens.next();
            let right = self.parse_multiplicative_expr();

            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::new(c),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_multiplicative_expr(&mut self) -> Expr {
        let mut left = self.parse_primary_expr();

        while let Some(&Token::BinaryOperator(c)) = self.tokens.peek() {
            if !matches!(c, '*' | '/' | '%') {
                break;
            }
            self.tokens.next();
            let right = self.parse_primary_expr();

            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::new(c),
                right: Box::new(right),
            };
        }

        left
    }
}
