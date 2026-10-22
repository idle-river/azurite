use std::collections::HashMap;

use crate::eval::evaluate_expr;
use ast::Expr;
use scopes::{RuntimeVal, Scope};

pub fn evaluate_object_expr(props: Vec<Box<Expr>>, env: &mut Scope) -> RuntimeVal {
    let mut properties = HashMap::new();

    for prop in props {
        let Expr::Properties { key, value } = *prop else {
            unreachable!()
        };

        let value = match value {
            Some(v) => evaluate_expr(*v, env),
            None => env.lookup_var(key.clone()).unwrap().clone(),
        };

        properties.insert(key, Box::new(value));
    }

    RuntimeVal::Object(properties)
}
