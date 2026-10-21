use crate::eval::evaluate_expr;
use ast::{BinaryOp, Expr};
use scopes::{RuntimeVal, Scope};

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

pub(crate) fn evaluate_binary_expr(
    left: Expr,
    op: BinaryOp,
    right: Expr,
    env: &mut Scope,
) -> RuntimeVal {
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
