#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Program { body: vec![] }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VariableDeclaration {
        is_const: bool,
        ident: String,
        value: Expr,
    },
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

    Assignment {
        assigne: Box<Expr>,
        value: Box<Expr>,
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
