mod values;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
pub use values::{Closure, FunctionCall, RuntimeVal};

/// A shared, mutable handle to a scope. Cloning it clones the handle (cheap),
/// not the scope, which is what lets closures keep their declaring scope alive.
pub type Env = Rc<RefCell<Scope>>;

#[derive(Debug)]
// Runtime Variables
pub struct Scope {
    parent: Option<Env>,
    variables: HashMap<String, RuntimeVal>,
    constants: HashSet<String>,
}

#[derive(Debug)]
pub enum ScopeError {
    VariableAlreadyDeclared {
        variable_name: String,
        declared_value: RuntimeVal,
    },
    VariableDoesNotExist(String),
    VariableInsertError(String),
    ReassignConstantError(String),
}

impl std::fmt::Display for ScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeError::VariableAlreadyDeclared {
                variable_name,
                declared_value,
            } => write!(
                f,
                "{} has already been declared; value is declared as {:?}",
                variable_name, declared_value
            ),
            ScopeError::VariableInsertError(v) => write!(f, "error inserting variable {v}"),
            ScopeError::VariableDoesNotExist(v) => write!(f, "variable {v} does not exist"),
            ScopeError::ReassignConstantError(c) => write!(f, "constant {c} cannot be reassigned"),
        }
    }
}

impl std::error::Error for ScopeError {}

impl Scope {
    /// A new, empty scope whose parent is `parent`.
    pub fn child(parent: &Env) -> Env {
        Rc::new(RefCell::new(Self {
            parent: Some(Rc::clone(parent)),
            variables: HashMap::new(),
            constants: HashSet::new(),
        }))
    }

    pub fn global() -> Env {
        let mut env = Self {
            parent: None,
            variables: HashMap::new(),
            constants: HashSet::new(),
        };

        // Runtime Variables
        declare_variable!(env, "true", RuntimeVal::Boolean(true), true);
        declare_variable!(env, "false", RuntimeVal::Boolean(false), true);
        declare_variable!(env, "null", RuntimeVal::Null, true);

        // Native Functions
        declare_native_fn!(env, "print", |args, _scope| {
            for arg in args {
                println!("{}", arg);
            }

            RuntimeVal::Null
        });

        Rc::new(RefCell::new(env))
    }

    // Making a new variable
    pub fn declare_var(&mut self, name: String, value: RuntimeVal) -> Result<(), ScopeError> {
        use std::collections::hash_map::Entry;

        match self.variables.entry(name) {
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
            Entry::Occupied(entry) => Err(ScopeError::VariableAlreadyDeclared {
                variable_name: entry.key().clone(),
                declared_value: entry.get().clone(),
            }),
        }
    }

    pub fn declare_const_var(&mut self, name: String, value: RuntimeVal) -> Result<(), ScopeError> {
        use std::collections::hash_map::Entry;

        match self.variables.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(value);
                self.constants.insert(name);
                Ok(())
            }
            Entry::Occupied(entry) => Err(ScopeError::VariableAlreadyDeclared {
                variable_name: entry.key().clone(),
                declared_value: entry.get().clone(),
            }),
        }
    }

    // Making a new native (builtin) function
    pub fn declare_native_fn(&mut self, name: &str, func: FunctionCall) -> Result<(), ScopeError> {
        let value = RuntimeVal::NativeFn(func);

        self.declare_const_var(name.to_string(), value)?;
        Ok(())
    }

    // Assigning to an already existing variable. Walks up the parent chain to
    // find the scope that owns the variable.
    pub fn assign_var(&mut self, name: String, value: RuntimeVal) -> Result<(), ScopeError> {
        if self.variables.contains_key(&name) {
            if self.constants.contains(&name) {
                return Err(ScopeError::ReassignConstantError(name));
            }

            self.variables.insert(name, value);
            Ok(())
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign_var(name, value)
        } else {
            Err(ScopeError::VariableDoesNotExist(name))
        }
    }

    // Returns an owned value: a reference can't outlive the RefCell borrow guard.
    pub fn lookup_var(&self, name: &str) -> Result<RuntimeVal, ScopeError> {
        if let Some(value) = self.variables.get(name) {
            return Ok(value.clone());
        }

        match &self.parent {
            Some(parent) => parent.borrow().lookup_var(name),
            None => Err(ScopeError::VariableDoesNotExist(name.to_string())),
        }
    }
}

#[macro_export]
macro_rules! declare_variable {
    ($env:expr, $ident: expr, $t:expr) => {
        $env.declare_var($ident.to_string(), $t).unwrap()
    };
    ($env:expr, $ident: expr, $t:expr, $constant: expr) => {{
        if $constant {
            $env.declare_const_var($ident.to_string(), $t).unwrap()
        } else {
            $env.declare_var($ident.to_string(), $t).unwrap()
        }
    }};
}

#[macro_export]
macro_rules! declare_native_fn {
    ($env:expr, $name:expr, |$args:ident, $scope:ident| $body:block) => {{
        $env.declare_native_fn(
            $name,
            |$args: Vec<$crate::RuntimeVal>, $scope: &$crate::Env| -> $crate::RuntimeVal { $body },
        )
        .unwrap()
    }};
}

#[cfg(test)]
mod tests;
