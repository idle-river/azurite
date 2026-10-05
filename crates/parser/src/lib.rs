use std::iter::Peekable;

use ast::{BinaryOperator, Expr, Program, Stmt};
use lexer::{Token, tokenize};

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! expect_identifer {
    ($x: expr) => {
        match $x.expect(|tok| matches!(tok, Token::Identifier(_))) {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        }
    };
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

    fn expect<F>(&mut self, predicate: F) -> Token
    where
        F: FnOnce(&Token) -> bool,
    {
        let token = self.eat();

        if predicate(&token) {
            token
        } else {
            panic!("Unexpected token: {:?}", token);
        }
    }

    fn parse_stmt(&mut self) -> Stmt {
        let tok = self.tokens.peek().unwrap();

        match tok {
            Token::Let | Token::Const => self.parse_var_decl(),
            _ => Stmt::Expression(self.parse_expr()),
        }
    }

    fn parse_var_decl(&mut self) -> Stmt {
        let token = self.eat();

        let is_const = token == Token::Const;
        let ident = expect_identifer!(self);

        self.expect(|tok| tok == &Token::Equals);
        let dec = Stmt::VariableDeclaration {
            is_const,
            ident,
            value: self.parse_expr(),
        };

        self.expect(|tok| tok == &Token::SemiColon);
        dec
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_assignment_expr()
    }

    fn parse_object_expr(&mut self) -> Expr {
        // { x: 100 }
        if self.tokens.peek() != Some(&Token::OpenBrace) {
            return self.parse_additive_expr();
        }

        self.eat(); // advanced past openbrace

        let mut properties = Vec::<Box<Expr>>::new();

        while self.not_eof() && self.tokens.peek() != Some(&Token::CloseBrace) {
            // { x: 100 }
            // { x: 100, }
            let key = expect_identifer!(self);

            // { x, }
            if self.tokens.peek() == Some(&Token::Comma) {
                self.eat(); // advance past comma
                properties.push(Box::new(Expr::Property { key, value: None }));
                continue;
            // { x }
            } else if self.tokens.peek() == Some(&Token::CloseBrace) {
                properties.push(Box::new(Expr::Property { key, value: None }));
                continue;
            }

            self.expect(|tok| tok == &Token::Colon);
            let value = Box::new(self.parse_expr());

            properties.push(Box::new(Expr::Property {
                key,
                value: Some(value),
            }));

            if self.tokens.peek() != Some(&Token::CloseBrace) {
                self.expect(|tok| tok == &Token::Comma);
            }
        }

        self.expect(|tok| tok == &Token::CloseBrace);
        Expr::Object(properties)
    }

    fn parse_assignment_expr(&mut self) -> Expr {
        let left = self.parse_object_expr();

        if self.tokens.peek() == Some(&Token::Equals) {
            self.eat();
            let value = self.parse_assignment_expr();
            Expr::Assignment {
                assigne: Box::new(left),
                value: Box::new(value),
            }
        } else {
            left
        }
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let token = self.eat();

        match token {
            Token::Identifier(ident) => Expr::Identifier(ident),
            Token::Number(num) => Expr::NumericLiteral(num.parse().unwrap()),
            Token::OpenParen => {
                let value = self.parse_expr();
                self.expect(|tok| tok == &Token::CloseParen);
                value
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numeric_literal_statement() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("42");

        assert_eq!(program.body.len(), 1);

        match &program.body[0] {
            Stmt::Expression(Expr::NumericLiteral(value)) => assert_eq!(*value, 42.0),
            other => panic!("expected numeric literal expression, got: {:?}", other),
        }
    }

    #[test]
    fn parses_identifier_statement() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("my_value");

        assert_eq!(program.body.len(), 1);

        match &program.body[0] {
            Stmt::Expression(Expr::Identifier(ident)) => assert_eq!(ident, "my_value"),
            other => panic!("expected identifier expression, got: {:?}", other),
        }
    }

    #[test]
    fn multiplicative_has_higher_precedence_than_additive() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("2 + 3 * 4");

        match &program.body[0] {
            Stmt::Expression(Expr::Binary {
                left,
                operator: BinaryOperator::Add,
                right,
            }) => {
                match left.as_ref() {
                    Expr::NumericLiteral(value) => assert_eq!(*value, 2.0),
                    other => panic!("expected left numeric literal, got: {:?}", other),
                }

                match right.as_ref() {
                    Expr::Binary {
                        left,
                        operator: BinaryOperator::Multiply,
                        right,
                    } => {
                        match left.as_ref() {
                            Expr::NumericLiteral(value) => assert_eq!(*value, 3.0),
                            other => {
                                panic!("expected right-left numeric literal, got: {:?}", other)
                            }
                        }
                        match right.as_ref() {
                            Expr::NumericLiteral(value) => assert_eq!(*value, 4.0),
                            other => {
                                panic!("expected right-right numeric literal, got: {:?}", other)
                            }
                        }
                    }
                    other => panic!("expected multiplication on right side, got: {:?}", other),
                }
            }
            other => panic!("expected additive binary expression, got: {:?}", other),
        }
    }

    #[test]
    fn parentheses_override_precedence() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("(2 + 3) * 4");

        match &program.body[0] {
            Stmt::Expression(Expr::Binary {
                left,
                operator: BinaryOperator::Multiply,
                right,
            }) => {
                match left.as_ref() {
                    Expr::Binary {
                        left,
                        operator: BinaryOperator::Add,
                        right,
                    } => {
                        match left.as_ref() {
                            Expr::NumericLiteral(value) => assert_eq!(*value, 2.0),
                            other => panic!("expected left-left numeric literal, got: {:?}", other),
                        }
                        match right.as_ref() {
                            Expr::NumericLiteral(value) => assert_eq!(*value, 3.0),
                            other => {
                                panic!("expected left-right numeric literal, got: {:?}", other)
                            }
                        }
                    }
                    other => panic!(
                        "expected additive expression inside parentheses, got: {:?}",
                        other
                    ),
                }

                match right.as_ref() {
                    Expr::NumericLiteral(value) => assert_eq!(*value, 4.0),
                    other => panic!("expected right numeric literal, got: {:?}", other),
                }
            }
            other => panic!(
                "expected multiplicative binary expression, got: {:?}",
                other
            ),
        }
    }

    #[test]
    fn additive_is_left_associative() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("10 - 3 - 2");

        match &program.body[0] {
            Stmt::Expression(Expr::Binary {
                left,
                operator: BinaryOperator::Subtract,
                right,
            }) => {
                match left.as_ref() {
                    Expr::Binary {
                        left,
                        operator: BinaryOperator::Subtract,
                        right,
                    } => {
                        match left.as_ref() {
                            Expr::NumericLiteral(value) => assert_eq!(*value, 10.0),
                            other => panic!("expected first numeric literal, got: {:?}", other),
                        }
                        match right.as_ref() {
                            Expr::NumericLiteral(value) => assert_eq!(*value, 3.0),
                            other => panic!("expected second numeric literal, got: {:?}", other),
                        }
                    }
                    other => panic!("expected left-associated subtraction, got: {:?}", other),
                }

                match right.as_ref() {
                    Expr::NumericLiteral(value) => assert_eq!(*value, 2.0),
                    other => panic!("expected final numeric literal, got: {:?}", other),
                }
            }
            other => panic!("expected subtraction expression, got: {:?}", other),
        }
    }

    #[test]
    fn parses_multiple_expression_statements() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("1 2");

        assert_eq!(program.body.len(), 2);
    }

    #[test]
    fn multiplicative_is_left_associative() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("20 / 5 / 2");

        match &program.body[0] {
            Stmt::Expression(Expr::Binary {
                left,
                operator: BinaryOperator::Divide,
                right,
            }) => {
                match left.as_ref() {
                    Expr::Binary {
                        left,
                        operator: BinaryOperator::Divide,
                        right,
                    } => {
                        match left.as_ref() {
                            Expr::NumericLiteral(value) => assert_eq!(*value, 20.0),
                            other => panic!("expected first numeric literal, got: {:?}", other),
                        }
                        match right.as_ref() {
                            Expr::NumericLiteral(value) => assert_eq!(*value, 5.0),
                            other => panic!("expected second numeric literal, got: {:?}", other),
                        }
                    }
                    other => panic!("expected left-associated division, got: {:?}", other),
                }

                match right.as_ref() {
                    Expr::NumericLiteral(value) => assert_eq!(*value, 2.0),
                    other => panic!("expected final numeric literal, got: {:?}", other),
                }
            }
            other => panic!("expected division expression, got: {:?}", other),
        }
    }

    #[test]
    fn parses_object_literal_with_shorthand_and_nested_object() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("const obj = { x: 100, foo, nested: { bar: true } };");

        match &program.body[0] {
            Stmt::VariableDeclaration {
                is_const: true,
                ident,
                value,
            } => {
                assert_eq!(ident, "obj");

                let Expr::Object(properties) = value else {
                    panic!("expected object literal value, got: {:?}", value);
                };

                assert_eq!(properties.len(), 3);
            }
            other => panic!("expected const variable declaration, got: {:?}", other),
        }
    }
}
