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

#[cfg(test)]
mod tests {
    use super::*;
    use ast::BinaryOperator;

    fn expression_stmt(expr: Expr) -> Stmt {
        Stmt::Expression(expr)
    }

    fn numeric(value: f64) -> Expr {
        Expr::NumericLiteral(value)
    }

    fn binary(left: Expr, operator: BinaryOperator, right: Expr) -> Expr {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    #[test]
    fn returns_last_expression_value() {
        let program = Program {
            body: vec![expression_stmt(numeric(1.0)), expression_stmt(numeric(2.0))],
        };

        assert_eq!(evaluate(program), RuntimeValue::Number(2.0));
    }

    #[test]
    fn evaluates_to_null_for_empty_program() {
        let program = Program::new();

        assert_eq!(evaluate(program), RuntimeValue::Null);
    }

    #[test]
    fn evaluates_all_numeric_binary_operators() {
        let program = Program {
            body: vec![
                expression_stmt(binary(numeric(6.0), BinaryOperator::Add, numeric(4.0))),
                expression_stmt(binary(numeric(6.0), BinaryOperator::Subtract, numeric(4.0))),
                expression_stmt(binary(numeric(6.0), BinaryOperator::Multiply, numeric(4.0))),
                expression_stmt(binary(numeric(8.0), BinaryOperator::Divide, numeric(4.0))),
                expression_stmt(binary(numeric(9.0), BinaryOperator::Modulo, numeric(4.0))),
            ],
        };

        assert_eq!(evaluate(program), RuntimeValue::Number(1.0));
    }

    #[test]
    fn null_in_binary_expression_propagates() {
        let program = Program {
            body: vec![expression_stmt(binary(
                Expr::NullLiteral,
                BinaryOperator::Add,
                numeric(1.0),
            ))],
        };

        assert_eq!(evaluate(program), RuntimeValue::Null);
    }

    #[test]
    fn evaluates_nested_binary_expressions() {
        let program = Program {
            body: vec![expression_stmt(binary(
                binary(numeric(2.0), BinaryOperator::Add, numeric(3.0)),
                BinaryOperator::Multiply,
                numeric(4.0),
            ))],
        };

        assert_eq!(evaluate(program), RuntimeValue::Number(20.0));
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn divide_by_zero_panics() {
        let program = Program {
            body: vec![expression_stmt(binary(
                numeric(10.0),
                BinaryOperator::Divide,
                numeric(0.0),
            ))],
        };

        let _ = evaluate(program);
    }
}
