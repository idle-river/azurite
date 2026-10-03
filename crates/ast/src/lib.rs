#[derive(Debug)]
pub struct Program {
    pub body: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Program { body: vec![] }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
}

#[derive(Debug)]
pub enum Expr {
    NumericLiteral(f64),
    Identifier(String),

    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
