use ast::{BinaryOp, Expr, Program, Stmt};
use scopes::RuntimeVal;

pub fn evaluate(program: Program) -> RuntimeVal {
    let mut last = RuntimeVal::Null; // an empty program evaluates to Null

    for stmt in program.statements {
        last = evaluate_stmt(stmt);
    }

    last
}

fn evaluate_stmt(stmt: Stmt) -> RuntimeVal {
    match stmt {
        Stmt::Expr(expr) => evaluate_expr(expr),
    }
}

fn evaluate_expr(expr: Expr) -> RuntimeVal {
    use std::process;
    match expr {
        Expr::Integer(i) => RuntimeVal::Integer(i),
        Expr::FloatingNumber(f) => RuntimeVal::FloatingNumber(f),
        Expr::Null => RuntimeVal::Null,
        Expr::Binary { left, op, right } => evaluate_binary_expr(*left, op, *right),
        _ => {
            eprintln!("AST Node: {:?} has not been setup for interpretation", expr);
            process::exit(1);
        }
    }
}

macro_rules! arith {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match ($lhs, $rhs) {
            (RuntimeVal::Integer(a), RuntimeVal::Integer(b)) => RuntimeVal::Integer(a $op b),
            (RuntimeVal::FloatingNumber(a), RuntimeVal::FloatingNumber(b)) => {
                RuntimeVal::FloatingNumber(a $op b)
            }
            (l, r) => panic!("mismatched operand types: {:?} and {:?}", l, r),
        }
    };
}

fn evaluate_binary_expr(left: Expr, op: BinaryOp, right: Expr) -> RuntimeVal {
    let lhs = evaluate_expr(left);
    let rhs = evaluate_expr(right);

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
