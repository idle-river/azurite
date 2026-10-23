use ast::Stmt;
use scopes::{RuntimeVal, Scope};

use crate::eval::evaluate_stmt;

pub fn eval_function_decl(func: Stmt, env: &mut Scope) -> RuntimeVal {
    let Stmt::FunctionDeclaration {
        parameters,
        name,
        body,
        is_async,
    } = func
    else {
        unreachable!()
    };

    let body: Vec<RuntimeVal> = body
        .iter()
        .map(|s| evaluate_stmt(*s.clone(), env))
        .collect();

    RuntimeVal::Function {
        name,
        parameters,
        dec_env: env,
        body,
    }
}
