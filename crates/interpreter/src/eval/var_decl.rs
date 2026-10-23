use crate::eval::evaluate_expr;
use ast::Expr;
use scopes::{Env, RuntimeVal};

pub(crate) fn eval_var_decl(
    constant: bool,
    identifier: String,
    value: Expr,
    env: &Env,
) -> RuntimeVal {
    let val = evaluate_expr(value, env);
    if constant {
        env.borrow_mut().declare_const_var(identifier, val).unwrap();
    } else {
        env.borrow_mut().declare_var(identifier, val).unwrap();
    }

    RuntimeVal::Null
}
