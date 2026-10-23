use ast::Expr;
use scopes::{Env, RuntimeVal};

use crate::eval::evaluate_expr;

pub fn evaluate_assign_expr(assigne: Expr, value: Expr, env: &Env) -> RuntimeVal {
    let Expr::Identifier(ident) = assigne else {
        eprintln!("Invalid LHS inside assignment expr: {:?}", assigne);
        return RuntimeVal::Null;
    };

    // Evaluate first: the RHS may touch `env`, so no borrow can be held yet.
    let value = evaluate_expr(value, env);
    env.borrow_mut().assign_var(ident.clone(), value).unwrap();

    let result = env.borrow().lookup_var(&ident).unwrap();
    result
}
