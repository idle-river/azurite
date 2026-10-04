#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Program { body: vec![] }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    NumericLiteral(f64),
    Identifier(String),

    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl BinaryOperator {
    pub fn new(op: char) -> Self {
        match op {
            '+' => BinaryOperator::Add,
            '-' => BinaryOperator::Subtract,
            '*' => BinaryOperator::Multiply,
            '/' => BinaryOperator::Divide,
            '%' => BinaryOperator::Modulo,
            _ => panic!("not a binary operator"),
        }
    }
}
