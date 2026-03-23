use std::collections::HashMap;
use crate::interpreter::Value;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_enclosed(parent: Environment) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn set(&mut self, name: String, val: Value) {
        self.values.insert(name, val);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.values.get(name) {
            return Some(v.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.get(name);
        }

        None
    }

    pub fn assign(&mut self, name: String, val: Value) -> Result<(), String> {
        if self.values.contains_key(&name) {
            self.values.insert(name, val);
            return Ok(());
        }

        if let Some(parent) = &mut self.parent {
            return parent.assign(name, val);
        }

        Err(format!("Undefined variable '{}'", name))
    }
}