use std::collections::HashMap;
use std::fmt;
use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::result;

use serde::{Deserialize, Serialize};

const FILENAME: &str = "store.db";
const COMPACT_LIMIT: i32 = 1_000;

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

#[derive(Debug)]
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
        KvStore::save(&self, &Command::Set(key, value))?;
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        match self.storage.get(&key) {
            Some(s) => Ok(Some(s.to_owned())),
            None => Ok(None),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        match self.get(key.to_owned())? {
            Some(_) => {
                self.storage.remove(&key);
                KvStore::save(&self, &Command::Remove(key))?;
                Ok(())
            }
            None => Err(KvError::KeyNotFound),
        }
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        let full_path = path.join(FILENAME);
        let mut store = KvStore::new();
        store.path = full_path.to_str().unwrap().to_string();

        if let Ok(file) = OpenOptions::new().read(true).open(&full_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(cmd) = line {
                    if let Ok(cmd) = serde_json::from_str::<Command>(&cmd) {
                        KvStore::load(&mut store, &cmd)?;
                    }
                }
            }
        }

        Ok(store)
    }

    fn load(store: &mut KvStore, command: &Command) -> Result<()> {
        match command {
            Command::Set(key, value) => {
                store.storage.insert(key.to_owned(), value.to_owned());
            }
            Command::Remove(key) => {
                store.storage.remove(key);
            }
        }

        Ok(())
    }

    fn save(store: &KvStore, command: &Command) -> Result<()> {
        let path = Path::new(&store.path);
        let path_count_str = format!("{}c", &store.path);
        let path_count = Path::new(&path_count_str);

        let mut count = 0;
        if let Ok(file) = OpenOptions::new().read(true).open(&path_count) {
            if let Ok(Command::Set(key, value)) = serde_json::from_reader::<File, Command>(file) {
                if key == "count" {
                    count = value.parse().unwrap_or(0);
                }
            }
        }

        // Do compaction
        if count > COMPACT_LIMIT {
            if let Ok(mut file) = OpenOptions::new().create(true).write(true).truncate(true).open(path) {
                for (k, v) in &store.storage {
                    KvStore::write_command(&mut file, &Command::Set(k.to_owned(), v.to_owned()))?;
                }

                // Reset count
                if let Ok(mut file) = OpenOptions::new().create(true).write(true).truncate(true).open(path_count) {
                    KvStore::write_command(&mut file, &Command::Set("count".to_owned(), "0".to_owned()))?;
                }
            }
        } else {
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                KvStore::write_command(&mut file, &command)?;

                // Increment count
                if let Ok(mut file) = OpenOptions::new().create(true).write(true).truncate(true).open(path_count) {
                    KvStore::write_command(&mut file, &Command::Set("count".to_owned(), (count + 1).to_string()))?;
                }
            }
        }

        Ok(())
    }

    fn write_command(file: &mut File, command: &Command) -> Result<()> {
        let cmd = serde_json::to_string(command).unwrap();
        match writeln!(file, "{}", cmd) {
            Ok(_) => Ok(()),
            Err(err) => Err(KvError::IoError(err.to_string())),
        }
    }
}