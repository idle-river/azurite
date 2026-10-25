mod error;
mod values;

use ast::{Expr, Program, Stmt};
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
        _ => Err(InterpreterError::NoNodeInterpretation(Stmt::Expression(
            expr,
        ))),
    }
}
