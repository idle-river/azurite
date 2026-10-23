mod assign_expr;
mod binary_expr;
mod call_expr;
mod expr;
mod function_decl;
mod object_expr;
mod stmt;
mod var_decl;

pub(crate) use assign_expr::evaluate_assign_expr;
pub(crate) use binary_expr::evaluate_binary_expr;
pub(crate) use call_expr::eval_call_expr;
pub(crate) use expr::evaluate_expr;
pub(crate) use function_decl::eval_function_decl;
pub(crate) use object_expr::evaluate_object_expr;
pub(crate) use stmt::evaluate_stmt;
pub(crate) use var_decl::eval_var_decl;
