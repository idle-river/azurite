mod error;
use scopes::{RuntimeVal, Scope};

use ast::{BinaryOp, Expr, Program, Stmt};
use error::InterpreterError;

pub fn evaluate(prog: Program, scope: &mut Scope) -> Result<RuntimeVal, InterpreterError> {
    let mut last_eval = RuntimeVal::Null;
    for stmt in prog.body {
        last_eval = eval_stmt(stmt, scope)?;
    }

    Ok(last_eval)
}

fn eval_stmt(stmt: Stmt, scope: &mut Scope) -> Result<RuntimeVal, InterpreterError> {
    match stmt {
        Stmt::Expression(expr) => eval_expr(expr, scope),
        Stmt::VariableDecleration { .. } => Err(InterpreterError::NoNodeInterpretation(stmt)),
    }
}

fn eval_expr(expr: Expr, scope: &mut Scope) -> Result<RuntimeVal, InterpreterError> {
    match expr {
        Expr::Number(num) => Ok(RuntimeVal::Number(num)),
        Expr::Binary { op, left, right } => eval_binary_expr(op, *left, *right, scope),
        Expr::Identifier(ident) => scope
            .lookup_variable(&ident)
            .ok_or(InterpreterError::ValNotFound(ident)),
    }
}

fn eval_binary_expr(
    op: BinaryOp,
    left: Expr,
    right: Expr,
    scope: &mut Scope,
) -> Result<RuntimeVal, InterpreterError> {
    let lhs = eval_expr(left, scope)?;
    let rhs = eval_expr(right, scope)?;

    if lhs == RuntimeVal::Null || rhs == RuntimeVal::Null {
        return Ok(RuntimeVal::Null);
    }

    let RuntimeVal::Number(lval) = lhs else {
        return Err(InterpreterError::InvalidTypes(lhs, rhs));
    };
    let RuntimeVal::Number(rval) = rhs else {
        return Err(InterpreterError::InvalidTypes(lhs, rhs));
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
