use crate::eval::{
    eval_call_expr, evaluate_assign_expr, evaluate_binary_expr, evaluate_object_expr,
};
use ast::Expr;
use scopes::{Env, RuntimeVal};

fn evaluate_member_expr(object: Expr, property: Expr, computed: bool, env: &Env) -> RuntimeVal {
    let object_value = evaluate_expr(object, env);

    let key = if computed {
        match property {
            Expr::Identifier(ident) => ident,
            other => evaluate_expr(other, env).to_string(),
        }
    } else {
        let Expr::Identifier(ident) = property else {
            unreachable!()
        };
        ident
    };

    let RuntimeVal::Object(map) = object_value else {
        panic!("cannot access member on non-object value");
    };

    map.get(&key)
        .map(|v| *v.clone())
        .unwrap_or(RuntimeVal::Null)
}

pub(crate) fn evaluate_expr(expr: Expr, env: &Env) -> RuntimeVal {
    match expr {
        Expr::Integer(i) => RuntimeVal::Integer(i),
        Expr::FloatingNumber(f) => RuntimeVal::FloatingNumber(f),
        Expr::Identifier(ident) => env.borrow().lookup_var(&ident).unwrap(),
        Expr::Binary { left, op, right } => evaluate_binary_expr(*left, op, *right, env),
        Expr::Assignment { assigne, value } => evaluate_assign_expr(*assigne, *value, env),
        Expr::Object(props) => {
            evaluate_object_expr(props.iter().map(|p| *p.clone()).collect(), env)
        }
        Expr::Call { .. } => eval_call_expr(expr, env),
        Expr::Properties { .. } => unreachable!(),
        Expr::Member {
            object,
            property,
            computed,
        } => evaluate_member_expr(*object, *property, computed, env),
    }
}
