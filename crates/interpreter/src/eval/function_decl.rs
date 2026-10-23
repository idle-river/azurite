use std::rc::Rc;

use ast::Stmt;
use scopes::{Closure, Env, RuntimeVal};

pub fn eval_function_decl(func: Stmt, env: &Env) -> RuntimeVal {
    let Stmt::FunctionDeclaration {
        parameters,
        name,
        body,
        is_async: _,
    } = func
    else {
        unreachable!()
    };

    let value = RuntimeVal::Function(Rc::new(Closure {
        name: name.clone(),
        parameters,
        dec_env: Rc::clone(env),
        body,
    }));

    env.borrow_mut()
        .declare_const_var(name.clone(), value)
        .unwrap();
    env.borrow().lookup_var(&name).unwrap()
}
