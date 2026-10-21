use crate::eval::{eval_var_decl, evaluate_expr};
use ast::Stmt;
use scopes::{RuntimeVal, Scope};

pub(crate) fn evaluate_stmt(stmt: Stmt, env: &mut Scope) -> RuntimeVal {
    match stmt {
        Stmt::Expr(expr) => evaluate_expr(expr, env),
        Stmt::VarDecl {
            constant,
            identifier,
            value,
        } => eval_var_decl(constant, identifier, value, env),
    }
}
