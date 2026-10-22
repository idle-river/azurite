use crate::eval::{
    eval_call_expr, evaluate_assign_expr, evaluate_binary_expr, evaluate_object_expr,
};
use ast::Expr;
use scopes::{RuntimeVal, Scope};

pub(crate) fn evaluate_expr(expr: Expr, env: &mut Scope) -> RuntimeVal {
    match expr {
        Expr::Integer(i) => RuntimeVal::Integer(i),
        Expr::FloatingNumber(f) => RuntimeVal::FloatingNumber(f),
        Expr::Identifier(ident) => env.lookup_var(ident).unwrap().clone(),
        Expr::Binary { left, op, right } => evaluate_binary_expr(*left, op, *right, env),
        Expr::Assignment { assigne, value } => evaluate_assign_expr(*assigne, *value, env),
        Expr::Object(props) => {
            evaluate_object_expr(props.iter().map(|p| *p.clone()).collect(), env)
        }
        Expr::Call { .. } => eval_call_expr(expr, env),
        Expr::Properties { .. } => unreachable!(),
        Expr::Member { .. } => todo!(),
    }
}
