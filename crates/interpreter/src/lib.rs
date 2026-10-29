mod error;
use std::collections::HashMap;

use scopes::{RuntimeVal, Scope};

use ast::{BinaryOp, Expr, Program, Stmt};
use error::InterpreterError;

pub fn evaluate(prog: Program, scope: &mut Scope) -> Result<RuntimeVal, InterpreterError> {
    let mut last_eval = RuntimeVal::Null;
    for stmt in prog.body {
        last_eval = eval_stmt(stmt, scope)?;
    }

    Ok(last_eval)
}

fn eval_stmt(stmt: Stmt, scope: &mut Scope) -> Result<RuntimeVal, InterpreterError> {
    match stmt {
        Stmt::Expression(expr) => eval_expr(expr, scope),
        Stmt::VariableDecleration {
            identifier,
            value,
            is_constant,
        } => eval_variable_decl(identifier, value, is_constant, scope),
    }
}

fn eval_expr(expr: Expr, scope: &mut Scope) -> Result<RuntimeVal, InterpreterError> {
    match expr {
        Expr::Number(num) => Ok(RuntimeVal::Number(num)),
        Expr::Binary { op, left, right } => eval_binary_expr(op, *left, *right, scope),
        Expr::Identifier(ident) => scope
            .lookup_variable(&ident)
            .ok_or(InterpreterError::ValNotFound(ident)),
        Expr::Reassignment { assigne, value } => eval_reassignment_expr(*assigne, *value, scope),
        Expr::Object(props) => {
            let props = props.into_iter().map(|p| *p).collect();
            eval_object_expr(props, scope)
        }
        Expr::Property { .. } => unreachable!(),
    }
}

fn eval_binary_expr(
    op: BinaryOp,
    left: Expr,
    right: Expr,
    scope: &mut Scope,
) -> Result<RuntimeVal, InterpreterError> {
    let lhs = eval_expr(left, scope)?;
    let rhs = eval_expr(right, scope)?;

    if lhs == RuntimeVal::Null || rhs == RuntimeVal::Null {
        return Ok(RuntimeVal::Null);
    }

    let RuntimeVal::Number(lval) = lhs else {
        return Err(InterpreterError::InvalidTypes(lhs, rhs));
    };
    let RuntimeVal::Number(rval) = rhs else {
        return Err(InterpreterError::InvalidTypes(lhs, rhs));
    };

    Ok(RuntimeVal::Number(match op {
        BinaryOp::Add => lval + rval,
        BinaryOp::Subtract => lval - rval,
        BinaryOp::Multiply => lval * rval,
        BinaryOp::Divide => {
            if rval == 0f64 {
                return Err(InterpreterError::DivideByZero(lhs));
            }

            if lval == 0f64 { lval } else { lval / rval }
        }
        BinaryOp::Modulo => lval % rval,
        BinaryOp::Exponent => lval.powf(rval),
    }))
}

fn eval_reassignment_expr(
    left: Expr,
    value: Expr,
    scope: &mut Scope,
) -> Result<RuntimeVal, InterpreterError> {
    use scopes::error::ScopeError;

    let right_val = eval_expr(value, scope)?;

    // Evntually, we'll support object and class rewriting
    let Expr::Identifier(ident) = left else {
        return Err(InterpreterError::InvalidAssigne(left));
    };

    match scope.reassign_variable(&ident, right_val) {
        Ok(_) => Ok(RuntimeVal::Null),
        Err(e) => Err(match e {
            ScopeError::VariableDoesNotExist(e) => InterpreterError::ValNotFound(e),
            ScopeError::ReassigningConstant(e) => InterpreterError::ConstantReassign(e),
            _ => unreachable!(),
        }),
    }
}

fn eval_object_expr(props: Vec<Expr>, scope: &mut Scope) -> Result<RuntimeVal, InterpreterError> {
    let mut val = HashMap::new();

    for prop in props {
        let Expr::Property { key, value } = prop else {
            unreachable!()
        };

        let res = if let Some(value) = value {
            eval_expr(*value, scope)?
        } else {
            unimplemented!()
        };

        val.insert(key, res);
    }

    Ok(RuntimeVal::Object(val))
}

fn eval_variable_decl(
    ident: String,
    value: Expr,
    constant: bool,
    scope: &mut Scope,
) -> Result<RuntimeVal, InterpreterError> {
    let evaled = eval_expr(value, scope)?;

    scope
        .declare_variable(ident.clone(), evaled, constant)
        .map_err(|_| InterpreterError::VariableAlreadyDefined(ident))?;

    Ok(RuntimeVal::Null)
}
