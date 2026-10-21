use crate::eval::evaluate_expr;
use ast::Expr;
use scopes::{RuntimeVal, Scope};

pub(crate) fn eval_var_decl(
    constant: bool,
    identifier: String,
    value: Expr,
    env: &mut Scope,
) -> RuntimeVal {
    let val = evaluate_expr(value, env);
    if constant {
        env.declare_const_var(identifier, val).unwrap();
    } else {
        env.declare_var(identifier, val).unwrap();
    }

    RuntimeVal::Null
}
