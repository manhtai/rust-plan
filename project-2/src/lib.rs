use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::result;

use serde::{Deserialize, Serialize};

pub enum KvError {
    KeyNotFound,
    IoError(String),
    SerdeError(String),
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set(String, String),
    Remove(String),
}

static FILENAME: &str = "target/store.db";

impl fmt::Debug for KvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KvError::KeyNotFound => write!(f, "Key not found"),
            KvError::IoError(err) => write!(f, "IO error: {}", err),
            KvError::SerdeError(err) => write!(f, "Serde error: {}", err),
        }
    }
}

pub type Result<T> = result::Result<T, KvError>;

pub struct KvStore {
    storage: HashMap<String, String>,
    path: String,
}

impl KvStore {
    pub fn new() -> Self {
        let storage = HashMap::new();
        KvStore { storage, path: FILENAME.to_string() }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.storage.insert(key.to_owned(), value.to_owned());
        KvStore::save(&self, &Command::Set(key, value));
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        match self.storage.get(&key) {
            Some(s) => Ok(Some(s.to_owned())),
            None => Err(KvError::KeyNotFound),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.get(key.to_owned())?;
        self.storage.remove(&key);
        KvStore::save(&self, &Command::Remove(key));
        Ok(())
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        match OpenOptions::new().read(true).create(true).open(path) {
            Ok(file) => {
                let mut store = KvStore::new();
                store.path = path.to_str().unwrap_or(FILENAME).to_string();

                let reader = BufReader::new(file);
                for line in reader.lines() {
                    match line {
                        Ok(cmd) => match serde_json::from_str::<Command>(&cmd) {
                            Ok(cmd) => {
                                KvStore::load(&mut store, &cmd);
                            }
                            _ => (),
                        },
                        _ => ()
                    }
                }

                Ok(store)
            }
            Err(err) => Err(KvError::IoError(err.to_string())),
        }
    }

    fn load(store: &mut KvStore, command: &Command) -> Result<()> {
        match command {
            Command::Set(key, value) => {
                store.set(key.to_owned(), value.to_owned());
            }
            Command::Remove(key) => {
                store.remove(key.to_owned());
            }
        }

        Ok(())
    }

    fn save(store: &KvStore, command: &Command) -> Result<()> {
        match OpenOptions::new().append(true).open(&store.path) {
            Ok(mut file) => {
                match serde_json::to_string(command) {
                    Ok(cmd) => {
                        match writeln!(file, "{}", cmd) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(KvError::IoError),
                        };
                        Ok(())
                    }
                    Err(err) => Err(KvError::IoError(err.to_string())),
                }
            }
            Err(err) => Err(KvError::IoError(err.to_string())),
        }
    }
}