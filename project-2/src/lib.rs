use std::collections::HashMap;

pub struct KvStore {
    storage: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        let storage = HashMap::new();
        KvStore { storage }
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.storage.insert(key, value)
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.storage.get(&key) {
            Some(s) => Some(s.to_owned()),
            None => None
        }
    }

    pub fn remove(&mut self, key: String) -> Option<String> {
        self.storage.remove(&key)
    }
}