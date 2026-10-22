use ast::Expr;
use scopes::{RuntimeVal, Scope};

use crate::eval::evaluate_expr;

pub(crate) fn eval_call_expr(expr: Expr, env: &mut Scope) -> RuntimeVal {
    let Expr::Call { args, calle } = expr else {
        unreachable!()
    };
    let Expr::Identifier(calle) = *calle else {
        unreachable!()
    };

    let args: Vec<RuntimeVal> = args
        .iter()
        .map(|arg| *arg.clone())
        .map(|arg| evaluate_expr(arg, env))
        .collect();

    let RuntimeVal::NativeFn(func) = env.lookup_var(calle).unwrap().clone() else {
        unreachable!()
    };

    func(args, env)
}
