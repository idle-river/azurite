pub mod values;

use ast::{Expr, Program, Stmt};
use values::RuntimeValue;

pub fn evaluate(program: Program) -> RuntimeValue {
    let mut last_value = RuntimeValue::Null;

    for stmt in program.body {
        last_value = eval_stmt(stmt);
    }

    last_value
}

fn eval_stmt(stmt: Stmt) -> RuntimeValue {
    match stmt {
        Stmt::Expression(expr) => eval_expr(expr),
    }
}

fn eval_expr(ast_node: Expr) -> RuntimeValue {
    match ast_node {
        Expr::NumericLiteral(num) => RuntimeValue::Number(num),
        Expr::NullLiteral => RuntimeValue::Null,
        _ => unimplemented!("AST Node has not been implemented for the interpreter yet"),
    }
}
