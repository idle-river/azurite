mod error;
mod values;

use ast::{BinaryOp, Expr, Program, Stmt};
use error::InterpreterError;
pub use values::RuntimeVal;

pub fn evaluate(prog: Program) -> Result<RuntimeVal, InterpreterError> {
    let mut last_eval = RuntimeVal::Null;
    for stmt in prog.body {
        last_eval = eval_stmt(stmt)?;
    }

    Ok(last_eval)
}

fn eval_stmt(stmt: Stmt) -> Result<RuntimeVal, InterpreterError> {
    match stmt {
        Stmt::Expression(expr) => eval_expr(expr),
    }
}

fn eval_expr(expr: Expr) -> Result<RuntimeVal, InterpreterError> {
    match expr {
        Expr::Number(num) => Ok(RuntimeVal::Number(num)),
        Expr::Binary { op, left, right } => eval_binary_expr(op, *left, *right),
        _ => Err(InterpreterError::NoNodeInterpretation(Stmt::Expression(
            expr,
        ))),
    }
}

fn eval_binary_expr(op: BinaryOp, left: Expr, right: Expr) -> Result<RuntimeVal, InterpreterError> {
    let lhs = eval_expr(left)?;
    let rhs = eval_expr(right)?;

    if lhs == RuntimeVal::Null || rhs == RuntimeVal::Null {
        return Ok(RuntimeVal::Null);
    }

    let RuntimeVal::Number(lval) = lhs else {
        unreachable!();
    };
    let RuntimeVal::Number(rval) = rhs else {
        unreachable!();
    };

    Ok(RuntimeVal::Number(match op {
        BinaryOp::Add => lval + rval,
        BinaryOp::Subtract => lval - rval,
        BinaryOp::Multiply => lval * rval,
        BinaryOp::Divide => {
            if rval == 0f64 {
                return Err(InterpreterError::DivideByZero(lhs));
            }

            if lval == 0f64 { lval } else { lval / rval }
        }
        BinaryOp::Modulo => lval % rval,
        BinaryOp::Exponent => lval.powf(rval),
    }))
}
