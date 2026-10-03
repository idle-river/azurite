#[derive(Debug)]
pub struct Program {
    body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    VariableDeclaration { name: String, initalizer: Expr },
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
