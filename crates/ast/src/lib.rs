#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Divide,
    Multiply,
    Modulo,
    Exponent,
}

impl BinaryOp {
    pub fn new(op: char) -> Self {
        match op {
            '+' => BinaryOp::Add,
            '-' => BinaryOp::Subtract,
            '/' => BinaryOp::Divide,
            '*' => BinaryOp::Multiply,
            '%' => BinaryOp::Modulo,
            '^' => BinaryOp::Exponent,
            _ => panic!("Unknown op type"),
        }
    }
}
