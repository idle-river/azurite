use crate::eval::{eval_function_decl, eval_var_decl, evaluate_expr};
use ast::Stmt;
use scopes::{Env, RuntimeVal};

pub(crate) fn evaluate_stmt(stmt: Stmt, env: &Env) -> RuntimeVal {
    match stmt {
        Stmt::Expr(expr) => evaluate_expr(expr, env),
        Stmt::VarDecl {
            constant,
            identifier,
            value,
        } => eval_var_decl(constant, identifier, value, env),
        Stmt::FunctionDeclaration { .. } => eval_function_decl(stmt, env),
    }
}
