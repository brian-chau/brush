use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.variables.iter()
    }
}
