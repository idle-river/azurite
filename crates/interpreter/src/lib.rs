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
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let lhs = eval_expr(*left);
            let rhs = eval_expr(*right);

            if lhs == RuntimeValue::Null || rhs == RuntimeValue::Null {
                RuntimeValue::Null
            } else {
                eval_numeric_binop(lhs, operator, rhs)
            }
        }
        _ => unimplemented!("AST Node has not been implemented for the interpreter yet"),
    }
}

fn eval_numeric_binop(
    lhs: RuntimeValue,
    operator: ast::BinaryOperator,
    rhs: RuntimeValue,
) -> RuntimeValue {
    use ast::BinaryOperator::*;

    let RuntimeValue::Number(lhs) = lhs else {
        unreachable!()
    };
    let RuntimeValue::Number(rhs) = rhs else {
        unreachable!()
    };

    let result = match operator {
        Add => lhs + rhs,
        Subtract => lhs - rhs,
        Multiply => lhs * rhs,
        Divide => {
            if rhs == 0f64 {
                panic!("Cannot divide by zero");
            } else if lhs == 0f64 {
                0f64
            } else {
                lhs / rhs
            }
        }
        Modulo => lhs % rhs,
    };

    RuntimeValue::Number(result)
}
