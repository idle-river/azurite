pub mod values;

use std::{
    collections::{HashMap, HashSet},
    panic,
};
use values::RuntimeValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeValue>,
    constants: HashSet<String>,
}

#[macro_export]
macro_rules! declare_var {
    ($env: expr, $name: expr, $value: expr) => {
        $env.declare_variable($name.to_string(), $value, false);
    };
    ($env: expr, $name: expr, $value: expr, $is_constant: expr) => {
        $env.declare_variable($name.to_string(), $value, $is_constant);
    };
}

impl Environment {
    pub fn new(parent: Option<Environment>) -> Self {
        let parent = parent.map(Box::new);

        let env = Environment {
            parent,
            variables: HashMap::new(),
            constants: HashSet::new(),
        };

        env
    }

    pub fn global() -> Self {
        let mut env = Self::new(None);

        declare_var!(env, "true", RuntimeValue::Boolean(true), true);
        declare_var!(env, "false", RuntimeValue::Boolean(false), true);
        declare_var!(env, "null", RuntimeValue::Null, true);

        env
    }

    fn contains(&mut self, name: &str) -> bool {
        if let Some(env) = self.resolve(name) {
            env.variables.contains_key(name)
        } else {
            false
        }
    }

    pub fn resolve(&mut self, name: &str) -> Option<&mut Environment> {
        if self.variables.contains_key(name) {
            Some(self)
        } else if let Some(parent) = &mut self.parent {
            parent.resolve(name)
        } else {
            None
        }
    }

    pub fn declare_variable(
        &mut self,
        name: String,
        value: RuntimeValue,
        constant: bool,
    ) -> RuntimeValue {
        if self.contains(&name) {
            panic!(
                "Cannot declare variable {}; it has already been defined.",
                name
            );
        }

        if constant {
            self.constants.insert(name.clone());
        }

        self.variables.insert(name, value);
        value
    }

    pub fn assign_variable(&mut self, name: String, value: RuntimeValue) -> RuntimeValue {
        let env = &mut self
            .resolve(&name)
            .unwrap_or_else(|| panic!("Unable to resolve variable {}", name));

        if env.constants.contains(&name) {
            panic!("Cannot reassign to variable {}; declared as constant", name);
        }

        if let Some(old_val) = env.variables.get_mut(&name) {
            *old_val = value;
            value
        } else {
            panic!(
                "Unable to assign value to variable {}; it has not been initalized yet",
                name
            );
        }
    }

    pub fn lookup(&mut self, name: &str) -> &RuntimeValue {
        let env = self
            .resolve(name)
            .unwrap_or_else(|| panic!("Unable to resolve variable {}", name));
        env.variables.get(name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_with_vars(parent: Option<Environment>, vars: &[(&str, RuntimeValue)]) -> Environment {
        let mut variables = HashMap::new();
        let constants = HashSet::new();
        for (name, value) in vars {
            variables.insert((*name).to_string(), value.clone());
        }

        Environment {
            parent: parent.map(Box::new),
            variables,
            constants,
        }
    }

    #[test]
    fn new_creates_environment_with_default_globals_and_no_parent() {
        let env = Environment::new(None);

        assert!(env.parent.is_none());
        assert_eq!(
            env.variables.get("true"),
            Some(&RuntimeValue::Boolean(true))
        );
        assert_eq!(
            env.variables.get("false"),
            Some(&RuntimeValue::Boolean(false))
        );
        assert_eq!(env.variables.get("null"), Some(&RuntimeValue::Null));
    }

    #[test]
    fn resolve_finds_variable_in_current_environment() {
        let mut env = env_with_vars(None, &[("x", RuntimeValue::Number(10.0))]);

        let resolved = env
            .resolve("x")
            .expect("x should resolve in current environment");

        assert!(resolved.variables.contains_key("x"));
    }

    #[test]
    fn resolve_walks_up_to_parent_environment() {
        let parent = env_with_vars(None, &[("x", RuntimeValue::Number(42.0))]);
        let mut child = env_with_vars(Some(parent), &[]);

        let resolved = child
            .resolve("x")
            .expect("x should resolve in parent environment");

        assert!(resolved.variables.contains_key("x"));
        assert_eq!(
            resolved.variables.get("x"),
            Some(&RuntimeValue::Number(42.0))
        );
    }

    #[test]
    fn resolve_returns_none_for_unknown_variable() {
        let mut env = Environment::new(None);

        let resolved = env.resolve("missing");

        assert!(resolved.is_none());
    }

    #[test]
    fn assign_variable_updates_value_in_resolved_environment() {
        let parent = env_with_vars(None, &[("x", RuntimeValue::Number(1.0))]);
        let mut child = env_with_vars(Some(parent), &[]);

        let assigned = child.assign_variable("x".to_string(), RuntimeValue::Number(7.0));

        assert_eq!(assigned, RuntimeValue::Number(7.0));
        assert_eq!(child.lookup("x"), &RuntimeValue::Number(7.0));
    }

    #[test]
    fn declare_variable_adds_new_identifier() {
        let mut env = Environment::new(None);

        let declared = env.declare_variable("y".to_string(), RuntimeValue::Number(3.0), false);

        assert_eq!(declared, RuntimeValue::Number(3.0));
        assert_eq!(env.lookup("y"), &RuntimeValue::Number(3.0));
    }

    #[test]
    fn lookup_returns_reference_to_runtime_value() {
        let mut env = env_with_vars(None, &[("value", RuntimeValue::Null)]);

        let value = env.lookup("value");

        assert_eq!(value, &RuntimeValue::Null);
    }
}
