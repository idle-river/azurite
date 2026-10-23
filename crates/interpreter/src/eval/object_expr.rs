use std::collections::HashMap;

use crate::eval::evaluate_expr;
use ast::Expr;
use scopes::{Env, RuntimeVal};

pub fn evaluate_object_expr(props: Vec<Expr>, env: &Env) -> RuntimeVal {
    let mut properties = HashMap::new();

    for prop in props {
        let Expr::Properties { key, value } = prop else {
            unreachable!()
        };

        let value = match value {
            Some(v) => evaluate_expr(*v, env),
            None => env.borrow().lookup_var(&key).unwrap(),
        };

        properties.insert(key, Box::new(value));
    }

    RuntimeVal::Object(properties)
}
