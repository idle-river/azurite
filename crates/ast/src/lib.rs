#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Identifier(String),
    FloatingNumber(f32),
    Integer(i32),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
}

impl BinaryOp {
    pub fn new(op: char) -> Self {
        match op {
            '+' => BinaryOp::Add,
            '-' => BinaryOp::Subtract,
            '*' => BinaryOp::Multiply,
            '/' => BinaryOp::Divide,
            '%' => BinaryOp::Modulus,
            _ => unreachable!("you should've checked if this was valid in the tokens"),
        }
    }
}
