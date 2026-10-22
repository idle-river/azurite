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

    pub fn expect(&mut self, predicate: impl FnOnce(&Token) -> bool, expected: &str) -> Token {
        use std::process::exit;

        match self.tokens.next() {
            Some(tok) if predicate(&tok) => tok,
            Some(other) => {
                eprintln!("Expected {}, found {:?}", expected, other);
                exit(1);
            }
            None => {
                eprintln!("Expected {}, found end of input", expected);
                exit(1);
            }
        }
    }

    fn parse_stmt(&mut self) -> Stmt {
        let tk = self.tokens.peek().unwrap();
        match tk {
            Token::Let | Token::Const => self.parse_var_decl(),
            _ => {
                let expr = self.parse_expr();
                if Some(&Token::Semicolon) == self.tokens.peek() {
                    self.tokens.next();
                }
                Stmt::Expr(expr)
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_assignment_expr()
    }

    fn parse_object_expr(&mut self) -> Expr {
        if Some(&Token::OpenBrace) != self.tokens.peek() {
            return self.parse_additive_expr();
        };
        self.tokens.next(); // consume `{`

        // Key: ident
        let mut props = Vec::<Box<Expr>>::new();
        while let Some(tok) = self.tokens.peek() {
            if tok == &Token::CloseBrace {
                break;
            }

            let Token::Identifier(key) =
                self.expect(|tok| matches!(tok, &Token::Identifier(_)), "an identifier")
            else {
                unreachable!()
            };

            // Shorthand: `foo,` or `foo }` (peek only, don't consume the comma)
            if matches!(self.tokens.peek(), Some(Token::Comma | Token::CloseBrace)) {
                props.push(Box::new(Expr::Properties { key, value: None }));
            } else {
                self.expect(|tok| tok == &Token::Colon, "a colon");

                let value = self.parse_expr();

                props.push(Box::new(Expr::Properties {
                    key,
                    value: Some(Box::new(value)),
                }));
            }

            // The only place a comma is consumed
            if Some(&Token::Comma) == self.tokens.peek() {
                self.tokens.next();
            } else {
                break;
            }
        }

        self.expect(|tok| tok == &Token::CloseBrace, "a closing brace");

        Expr::Object(props)
    }

    fn parse_assignment_expr(&mut self) -> Expr {
        let left = self.parse_object_expr();

        if Some(&Token::Equals) == self.tokens.peek() {
            self.tokens.next().unwrap(); // advance past equals
            let value = self.parse_additive_expr();
            Expr::Assignment {
                assigne: Box::new(left),
                value: Box::new(value),
            }
        } else {
            left
        }
    }

    fn parse_var_decl(&mut self) -> Stmt {
        let is_constant = self.tokens.next().unwrap() == Token::Const;
        let Token::Identifier(ident) =
            self.expect(|tok| matches!(tok, &Token::Identifier(_)), "an identifier")
        else {
            unreachable!()
        };

        self.expect(|tok| tok == &Token::Equals, "an equals");

        let val = self.parse_expr();
        let msg = format!("a semicolon (line {})", line!() + 1);
        self.expect(|tok| tok == &Token::Semicolon, &msg);

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
                self.expect(|tok| tok == &Token::CloseParen, "a closing paren");

                expr
            }

            _ => {
                eprintln!("Unexpected token found during parsing: {:?}", tk);
                for tok in self.tokens.by_ref() {
                    println!("Token: {:#?}", tok);
                }
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
        let mut left = self.parse_call_member_expr();

        while let Some(&Token::BinaryOperator(c)) = self.tokens.peek() {
            if !matches!(c, '*' | '/' | '%') {
                break;
            }
            self.tokens.next();
            let right = self.parse_call_member_expr();

            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::new(c),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_call_member_expr(&mut self) -> Expr {
        let member = self.parse_member_expr();

        if Some(&Token::OpenParen) == self.tokens.peek() {
            self.parse_call_expr(member)
        } else {
            member
        }
    }

    fn parse_call_expr(&mut self, calle: Expr) -> Expr {
        let args = self
            .parse_args()
            .iter()
            .map(|arg| Box::new(arg.clone()))
            .collect();

        let mut call_expr = Expr::Call {
            args,
            calle: Box::new(calle),
        };

        if Some(&Token::OpenParen) == self.tokens.peek() {
            call_expr = self.parse_call_expr(call_expr);
        }

        call_expr
    }

    fn parse_args(&mut self) -> Vec<Expr> {
        self.expect(|tok| tok == &Token::OpenParen, "an opening paren");

        let args: Vec<Expr> = if Some(&Token::CloseParen) == self.tokens.peek() {
            Vec::<Expr>::new()
        } else {
            self.parse_arguments_list()
        };

        self.expect(|tok| tok == &Token::CloseParen, "a closing paren");

        args
    }

    fn parse_arguments_list(&mut self) -> Vec<Expr> {
        let mut args = vec![self.parse_assignment_expr()];

        while let Some(tok) = self.tokens.peek() {
            if tok == &Token::Comma {
                self.tokens.next();
                args.push(self.parse_assignment_expr());
            } else {
                break;
            }
        }

        args
    }

    fn parse_member_expr(&mut self) -> Expr {
        use std::process::exit;

        let mut obj = self.parse_primary_expr();

        while let Some(tok) = self.tokens.peek() {
            if tok == &Token::Dot || tok == &Token::OpenBracket {
                let op = self.tokens.next().unwrap();
                let property: Expr;
                let computed: bool;

                if op == Token::Dot {
                    computed = false;
                    property = self.parse_primary_expr();

                    if !matches!(property, Expr::Identifier(_)) {
                        eprintln!(
                            "Cannot use dot operator without right hand side being an identifier"
                        );
                        exit(1);
                    }
                } else {
                    computed = true;
                    property = self.parse_expr();
                    self.expect(|tok| tok == &Token::CloseBracket, "a closing bracket");
                }

                obj = Expr::Member {
                    object: Box::new(obj),
                    property: Box::new(property),
                    computed,
                };
            } else {
                break;
            }
        }

        obj
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_var_decl_without_hanging() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("const foo = 46;").unwrap();

        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn parses_member_expression_without_hanging() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("foo.bar").unwrap();

        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn parses_call_expression_without_panicking() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("foo();").unwrap();

        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn parses_call_expression_with_arguments() {
        let mut parser = Parser::new();
        let program = parser.produce_ast("foo(1, 2 + 3);").unwrap();

        assert_eq!(program.statements.len(), 1);
    }
}
