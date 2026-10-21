use ast::{BinaryOp, Expr, Program, Stmt};
use scopes::{RuntimeVal, Scope};

pub fn evaluate(program: Program, env: &mut Scope) -> RuntimeVal {
    let mut last = RuntimeVal::Null; // an empty program evaluates to Null

    for stmt in program.statements {
        last = evaluate_stmt(stmt, env);
    }

    last
}

fn evaluate_stmt(stmt: Stmt, env: &mut Scope) -> RuntimeVal {
    match stmt {
        Stmt::Expr(expr) => evaluate_expr(expr, env),
        Stmt::VarDecl {
            constant,
            identifier,
            value,
        } => eval_var_decl(constant, identifier, value, env),
    }
}

fn eval_var_decl(constant: bool, identifier: String, value: Expr, env: &mut Scope) -> RuntimeVal {
    let val = evaluate_expr(value, env);
    if constant {
        env.declare_const_var(identifier, val).unwrap();
    } else {
        env.declare_var(identifier, val).unwrap();
    }

    RuntimeVal::Null
}

fn evaluate_expr(expr: Expr, env: &mut Scope) -> RuntimeVal {
    match expr {
        Expr::Integer(i) => RuntimeVal::Integer(i),
        Expr::FloatingNumber(f) => RuntimeVal::FloatingNumber(f),
        Expr::Identifier(ident) => env.lookup_var(ident).unwrap().clone(),
        Expr::Binary { left, op, right } => evaluate_binary_expr(*left, op, *right, env),
    }
}

macro_rules! arith {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match ($lhs, $rhs) {
            (RuntimeVal::Integer(a), RuntimeVal::Integer(b)) => RuntimeVal::FloatingNumber((a as f64) $op (b as f64)),
            (RuntimeVal::FloatingNumber(a), RuntimeVal::FloatingNumber(b)) => {
                RuntimeVal::FloatingNumber(a $op b)
            }
            (l, r) => panic!("mismatched operand types: {:?} and {:?}", l, r),
        }
    };
}

fn evaluate_binary_expr(left: Expr, op: BinaryOp, right: Expr, env: &mut Scope) -> RuntimeVal {
    let lhs = evaluate_expr(left, env);
    let rhs = evaluate_expr(right, env);

    if lhs == RuntimeVal::Null || rhs == RuntimeVal::Null {
        return RuntimeVal::Null;
    }

    match op {
        BinaryOp::Add => arith!(lhs, rhs, +),
        BinaryOp::Subtract => arith!(lhs, rhs, -),
        BinaryOp::Multiply => arith!(lhs, rhs, *),
        BinaryOp::Divide => {
            if lhs == RuntimeVal::Integer(0) || lhs == RuntimeVal::FloatingNumber(0.0) {
                return RuntimeVal::FloatingNumber(0.0);
            }

            if rhs == RuntimeVal::Integer(0) || lhs == RuntimeVal::FloatingNumber(0.0) {
                panic!("rhs division by 0");
            }

            arith!(lhs, rhs, /)
        }
        BinaryOp::Modulus => arith!(lhs, rhs, %),
    }
}
