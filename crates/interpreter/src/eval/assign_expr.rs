use ast::Expr;
use scopes::{RuntimeVal, Scope};

use crate::eval::evaluate_expr;

pub fn evaluate_assign_expr(assigne: Expr, value: Expr, env: &mut Scope) -> RuntimeVal {
    let Expr::Identifier(ident) = assigne else {
        eprintln!("Invalid LHS inside assignment expr: {:?}", assigne);
        return RuntimeVal::Null;
    };

    let value = evaluate_expr(value, env);
    env.assign_var(ident.clone(), value).unwrap();
    env.lookup_var(ident).unwrap().clone()
}
