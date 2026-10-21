use ast::{Expr, Program, Stmt};

use crate::values::RuntimeVal;

mod values;

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
        _ => {
            eprintln!("AST Node: {:?} has not been setup for interpretation", expr);
            process::exit(1);
        }
    }
}
