use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: String) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.values.get(name).cloned()
    }
}