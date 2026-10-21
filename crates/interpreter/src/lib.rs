use ast::Program;
use scopes::{RuntimeVal, Scope};

mod eval;
use eval::evaluate_stmt;

pub fn evaluate(program: Program, env: &mut Scope) -> RuntimeVal {
    let mut last = RuntimeVal::Null; // an empty program evaluates to Null

    for stmt in program.statements {
        last = evaluate_stmt(stmt, env);
    }

    last
}
