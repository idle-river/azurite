use interpreter::values::RuntimeValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeValue>,
}

impl Environment {
    pub fn new(parent: Option<Environment>) -> Self {
        let parent = if let Some(env) = parent {
            Some(Box::new(env))
        } else {
            None
        };

        Environment {
            parent,
            variables: HashMap::new(),
        }
    }

    fn contains(&mut self, name: &str) -> bool {
        self.resolve(&name).variables.contains_key(name)
    }

    pub fn resolve(&mut self, name: &str) -> &mut Environment {
        if self.variables.contains_key(name) {
            self
        } else if let Some(parent) = &mut self.parent {
            parent.resolve(name)
        } else {
            panic!("Unable to resolve variable {}", name);
        }
    }

    pub fn declare_variable(&mut self, name: String, value: RuntimeValue) -> RuntimeValue {
        if self.contains(&name) {
            panic!(
                "Cannot declare variable {}; it has already been defined.",
                name
            );
        }

        self.variables.insert(name, value.clone()).unwrap()
    }

    pub fn assign_variable(&mut self, name: String, value: RuntimeValue) -> RuntimeValue {
        let env = &mut self.resolve(&name);

        if let Some(old_val) = env.variables.get_mut(&name) {
            *old_val = value.clone();
            value
        } else {
            panic!(
                "Unable to assign value to variable {}; it has not been initalized yet",
                name
            );
        }
    }

    pub fn lookup(&mut self, name: &str) -> &RuntimeValue {
        let env = self.resolve(name);
        env.variables.get(name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_with_vars(parent: Option<Environment>, vars: &[(&str, RuntimeValue)]) -> Environment {
        let mut variables = HashMap::new();
        for (name, value) in vars {
            variables.insert((*name).to_string(), value.clone());
        }

        Environment {
            parent: parent.map(Box::new),
            variables,
        }
    }

    #[test]
    fn new_creates_empty_environment_without_parent() {
        let env = Environment::new(None);

        assert!(env.parent.is_none());
        assert!(env.variables.is_empty());
    }

    #[test]
    fn resolve_finds_variable_in_current_environment() {
        let mut env = env_with_vars(None, &[("x", RuntimeValue::Number(10.0))]);

        let resolved = env.resolve("x");

        assert!(resolved.variables.contains_key("x"));
    }

    #[test]
    fn resolve_walks_up_to_parent_environment() {
        let parent = env_with_vars(None, &[("x", RuntimeValue::Number(42.0))]);
        let mut child = env_with_vars(Some(parent), &[]);

        let resolved = child.resolve("x");

        assert!(resolved.variables.contains_key("x"));
        assert_eq!(
            resolved.variables.get("x"),
            Some(&RuntimeValue::Number(42.0))
        );
    }

    #[test]
    #[should_panic(expected = "Unable to resolve variable missing")]
    fn resolve_panics_for_unknown_variable() {
        let mut env = Environment::new(None);

        let _ = env.resolve("missing");
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
    #[should_panic(expected = "Unable to resolve variable y")]
    fn declare_variable_panics_for_new_identifier() {
        let mut env = Environment::new(None);

        let _ = env.declare_variable("y".to_string(), RuntimeValue::Number(3.0));
    }

    #[test]
    fn lookup_returns_reference_to_runtime_value() {
        let mut env = env_with_vars(None, &[("value", RuntimeValue::Null)]);

        let value = env.lookup("value");

        assert_eq!(value, &RuntimeValue::Null);
    }
}
