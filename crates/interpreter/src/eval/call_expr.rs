use ast::Expr;
use scopes::{Env, RuntimeVal, Scope};

use crate::eval::{evaluate_expr, evaluate_stmt};

pub(crate) fn eval_call_expr(expr: Expr, env: &Env) -> RuntimeVal {
    let Expr::Call { args, calle } = expr else {
        unreachable!()
    };

    let args: Vec<RuntimeVal> = args
        .iter()
        .map(|arg| *arg.clone())
        .map(|arg| evaluate_expr(arg, env))
        .collect();

    match evaluate_expr(*calle, env) {
        RuntimeVal::NativeFn(func) => func(args, env),
        RuntimeVal::Function(closure) => {
            let fn_env = Scope::child(&closure.dec_env);

            for (param, arg) in closure.parameters.iter().zip(args.into_iter()) {
                fn_env.borrow_mut().declare_var(param.clone(), arg).unwrap();
            }

            let mut last = RuntimeVal::Null;
            for stmt in closure.body.iter() {
                last = evaluate_stmt(*stmt.clone(), &fn_env);
            }

            last
        }
        val => panic!("cannot call value: {:?}", val),
    }
}
