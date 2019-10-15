use std::collections::HashMap;
use std::path::Path;
use std::result;


#[derive(Debug)]
pub enum KvError {
    KeyNotFound,
    IoError,
}

pub type Result<T> = result::Result<T, KvError>;

pub struct KvStore {
    storage: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        let storage = HashMap::new();
        KvStore { storage }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.storage.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        let result = match self.storage.get(&key) {
            Some(s) => Some(s.to_owned()),
            None => None
        };
        Ok(result)
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        Ok(KvStore::new())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.storage.remove(&key);
        Ok(())
    }
}