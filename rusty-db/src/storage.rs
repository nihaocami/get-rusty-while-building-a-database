use std::collections::HashMap;

struct Database {
    data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}
