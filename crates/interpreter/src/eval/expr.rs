use crate::eval::evaluate_binary_expr;
use ast::Expr;
use scopes::{RuntimeVal, Scope};

pub(crate) fn evaluate_expr(expr: Expr, env: &mut Scope) -> RuntimeVal {
    match expr {
        Expr::Integer(i) => RuntimeVal::Integer(i),
        Expr::FloatingNumber(f) => RuntimeVal::FloatingNumber(f),
        Expr::Identifier(ident) => env.lookup_var(ident).unwrap().clone(),
        Expr::Binary { left, op, right } => evaluate_binary_expr(*left, op, *right, env),
    }
}
