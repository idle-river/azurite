#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    VarDecl {
        constant: bool,
        identifier: String,
        value: Expr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Assignment {
        // Expr bc you might also need to eval stuff like
        // foo.bar
        // foo["bar"]
        assigne: Box<Expr>,
        value: Box<Expr>,
    },
    Member {
        object: Box<Expr>,
        property: Box<Expr>,
        computed: bool,
    },
    Call {
        args: Vec<Box<Expr>>,
        calle: Box<Expr>,
    },
    Identifier(String),
    FloatingNumber(f64),
    Integer(i32),
    Object(Vec<Box<Expr>>),
    Properties {
        key: String,
        value: Option<Box<Expr>>,
    },
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
