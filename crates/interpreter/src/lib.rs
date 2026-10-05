use ast::{Expr, Program, Stmt};
use environment::Environment;
use environment::values::RuntimeValue;

pub fn evaluate(program: Program, env: &mut Environment) -> RuntimeValue {
    let mut last_value = RuntimeValue::Null;

    for stmt in program.body {
        last_value = eval_stmt(stmt, env);
    }

    last_value
}

fn eval_stmt(stmt: Stmt, env: &mut Environment) -> RuntimeValue {
    match stmt {
        Stmt::Expression(expr) => eval_expr(expr, env),
        expr @ Stmt::VariableDeclaration { .. } => eval_var_declaration(expr, env),
    }
}

fn eval_expr(ast_node: Expr, env: &mut Environment) -> RuntimeValue {
    match ast_node {
        Expr::NumericLiteral(num) => RuntimeValue::Number(num),
        Expr::Identifier(ident) => *env.lookup(&ident),
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let lhs = eval_expr(*left, env);
            let rhs = eval_expr(*right, env);

            if lhs == RuntimeValue::Null || rhs == RuntimeValue::Null {
                RuntimeValue::Null
            } else {
                eval_numeric_binop(lhs, operator, rhs)
            }
        }
        Expr::Assignment { assigne, value } => {
            let Expr::Identifier(name) = *assigne else {
                panic!("Invalid LHS inside assignment expr {:#?}", *assigne);
            };

            let result = eval_expr(*value, env);
            env.assign_variable(name, result)
        }
        _ => {
            println!("AST Node: {:#?}", ast_node);
            unimplemented!("This AST node has not been setup.");
        }
    }
}

fn eval_numeric_binop(
    lhs: RuntimeValue,
    operator: ast::BinaryOperator,
    rhs: RuntimeValue,
) -> RuntimeValue {
    use ast::BinaryOperator::*;

    let RuntimeValue::Number(lhs) = lhs else {
        return RuntimeValue::Null;
    };
    let RuntimeValue::Number(rhs) = rhs else {
        return RuntimeValue::Null;
    };

    let result = match operator {
        Add => lhs + rhs,
        Subtract => lhs - rhs,
        Multiply => lhs * rhs,
        Divide => {
            if rhs == 0.0 {
                panic!("Cannot divide by zero");
            } else if lhs == 0.0 {
                0.0
            } else {
                lhs / rhs
            }
        }
        Modulo => lhs % rhs,
    };

    RuntimeValue::Number(result)
}

fn eval_var_declaration(var_decl: Stmt, env: &mut Environment) -> RuntimeValue {
    let Stmt::VariableDeclaration {
        is_const,
        ident,
        value,
    } = var_decl
    else {
        unreachable!();
    };

    let value: RuntimeValue = eval_expr(value, env);

    env.declare_variable(ident, value, is_const)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::BinaryOperator;
    use environment::Environment;
    use environment::values::RuntimeValue;

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

    fn evaluate_program(program: Program) -> RuntimeValue {
        let mut env = Environment::new(None);
        evaluate(program, &mut env)
    }

    #[test]
    fn returns_last_expression_value() {
        let program = Program {
            body: vec![expression_stmt(numeric(1.0)), expression_stmt(numeric(2.0))],
        };

        assert_eq!(evaluate_program(program), RuntimeValue::Number(2.0));
    }

    #[test]
    fn evaluates_to_null_for_empty_program() {
        let program = Program::new();

        assert_eq!(evaluate_program(program), RuntimeValue::Null);
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

        assert_eq!(evaluate_program(program), RuntimeValue::Number(1.0));
    }

    #[test]
    fn null_in_binary_expression_propagates() {
        let program = Program {
            body: vec![expression_stmt(binary(
                Expr::Identifier("null".to_string()),
                BinaryOperator::Add,
                numeric(1.0),
            ))],
        };

        assert_eq!(evaluate_program(program), RuntimeValue::Null);
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

        assert_eq!(evaluate_program(program), RuntimeValue::Number(20.0));
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

        let _ = evaluate_program(program);
    }
}
