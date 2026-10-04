use std::iter::Peekable;

use ast::{BinaryOperator, Expr, Program, Stmt};
use lexer::{Token, tokenize};

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            tokens: vec![].into_iter().peekable(),
        }
    }

    pub fn produce_ast(&mut self, source_code: &str) -> Program {
        let mut program = Program::new();

        self.tokens = tokenize(source_code).into_iter().peekable();

        // Parse til end of file
        while self.not_eof() {
            program.body.push(self.parse_stmt());
        }

        program
    }

    fn not_eof(&mut self) -> bool {
        self.tokens.peek().unwrap() != &Token::EOF
    }

    fn eat(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    fn parse_stmt(&mut self) -> Stmt {
        // no other stmts beside an expr
        Stmt::Expression(self.parse_expr())
    }

    fn parse_expr(&mut self) -> Expr {
        // just implementing additive exprs
        self.parse_additive_expr()
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let token = self.eat();

        match token {
            Token::Identifier(ident) => Expr::Identifier(ident),
            Token::Number(num) => Expr::NumericLiteral(num.parse().unwrap()),
            _ => panic!("Unexpected token found during parsing: {:?}", token),
        }
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let mut left = self.parse_multiplictive_expr();

        while let Some(Token::BinaryOperator(op @ ('+' | '-'))) = self.tokens.peek() {
            let op = *op;
            self.eat();

            let right = self.parse_multiplictive_expr();

            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOperator::new(op),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_multiplictive_expr(&mut self) -> Expr {
        let mut left = self.parse_primary_expr();

        while let Some(Token::BinaryOperator(op @ ('*' | '/' | '%'))) = self.tokens.peek() {
            let op = *op;
            self.eat();

            let right = self.parse_primary_expr();

            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOperator::new(op),
                right: Box::new(right),
            };
        }

        left
    }
}

// Orders of Prescidence (High -> Low)
// PrimaryExpr
// UnaryExpr
// MultiplicitiveExpr
// AdditiveExpr
// ComparisonExpr
// LogicalExpr
// FunctionCall
// MemberExpr
// AssignmentExpr
