use std::collections::HashMap;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, Read};
use std::path::Path;
use std::result;
use sled::Db;


use serde::{Deserialize, Serialize};
use failure::_core::str::from_utf8;

const FILENAME: &str = "db";
const COMPACT_LIMIT: i32 = 1_000;

#[derive(Serialize, Deserialize)]
pub enum KvError {
    KeyNotFound,
    IoError(String),
    SerdeError(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KvsCommand {
    Set(String, String),
    Remove(String),
    Get(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KvsResult {
    Some(String),
    None,
    Error(KvError),
    Ok,
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

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KvError::KeyNotFound => write!(f, "Key not found"),
            KvError::IoError(err) => write!(f, "IO error: {}", err),
            KvError::SerdeError(err) => write!(f, "Serde error: {}", err),
        }
    }
}

pub type Result<T> = result::Result<T, KvError>;

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&mut self, key: String) -> Result<Option<String>>;
    fn remove(&mut self, key: String) -> Result<()>;
    fn open(path: &Path) -> Result<Self> where Self: Sized;
}

#[derive(Debug, Default)]
pub struct KvStore {
    storage: HashMap<String, String>,
    path: String,
}

pub struct SledKvsEngine {
    storage: Db,
}

impl KvStore {
    fn new() -> Self {
        let storage = HashMap::new();
        KvStore {
            storage,
            path: FILENAME.to_string(),
        }
    }

    fn load(store: &mut KvStore, command: &KvsCommand) -> Result<()> {
        match command {
            KvsCommand::Set(key, value) => {
                store.storage.insert(key.to_owned(), value.to_owned());
            }
            KvsCommand::Remove(key) => {
                store.storage.remove(key);
            }
            _ => (),
        }

        Ok(())
    }

    fn save(store: &KvStore, command: &KvsCommand) -> Result<()> {
        let path = Path::new(&store.path);
        let path_count_str = format!("{}-count", &store.path);
        let path_count = Path::new(&path_count_str);

        let mut count = 0;
        if let Ok(file) = OpenOptions::new().read(true).open(&path_count) {
            if let Ok(KvsCommand::Set(key, value)) = serde_json::from_reader::<File, KvsCommand>(file) {
                if key == "count" {
                    count = value.parse().unwrap_or(0);
                }
            }
        }

        // Do compaction
        if count > COMPACT_LIMIT {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
            {
                for (k, v) in &store.storage {
                    KvStore::write_command(&mut file, &KvsCommand::Set(k.to_owned(), v.to_owned()))?;
                }

                // Reset count
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path_count)
                {
                    KvStore::write_command(
                        &mut file,
                        &KvsCommand::Set("count".to_owned(), "0".to_owned()),
                    )?;
                }
            }
        } else if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            KvStore::write_command(&mut file, &command)?;

            // Increment count
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path_count)
            {
                KvStore::write_command(
                    &mut file,
                    &KvsCommand::Set("count".to_owned(), (count + 1).to_string()),
                )?;
            }
        }

        Ok(())
    }

    fn write_command(file: &mut File, command: &KvsCommand) -> Result<()> {
        let cmd = serde_json::to_string(command).unwrap();
        match writeln!(file, "{}", cmd) {
            Ok(_) => Ok(()),
            Err(err) => Err(KvError::IoError(err.to_string())),
        }
    }
}

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.storage.insert(key.to_owned(), value.to_owned());
        KvStore::save(&self, &KvsCommand::Set(key, value))?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.storage.get(&key) {
            Some(s) => Ok(Some(s.to_owned())),
            None => Ok(None),
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        match self.get(key.to_owned())? {
            Some(_) => {
                self.storage.remove(&key);
                KvStore::save(&self, &KvsCommand::Remove(key))?;
                Ok(())
            }
            None => Err(KvError::KeyNotFound),
        }
    }

    fn open(path: &Path) -> Result<KvStore> {
        let full_path = path.join(FILENAME);
        let mut store = KvStore::new();
        store.path = full_path.to_str().unwrap().to_string();

        if !full_path.exists() {
            store.set("".to_owned(), "".to_owned());
            return Ok(store);
        } else {
            let first_10_bytes_of_kvs = &[123, 34, 83, 101, 116, 34, 58, 91, 34, 34];
            let mut file = OpenOptions::new().read(true).open(&full_path).unwrap();
            let mut first_bytes = [0; 10];
            file.read(&mut first_bytes);
            if !first_bytes.starts_with(first_10_bytes_of_kvs) {
                return Err(KvError::IoError("Unable to open!".to_owned()));
            }
        }

        if let Ok(file) = OpenOptions::new().read(true).open(&full_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(cmd) = line {
                    if let Ok(cmd) = serde_json::from_str::<KvsCommand>(&cmd) {
                        KvStore::load(&mut store, &cmd);
                    };
                }
            }
            return Ok(store);
        }

        return Err(KvError::IoError("Unable to open!".to_owned()));
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        &self.storage.insert(key.into_bytes(), value.into_bytes());
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        match &self.storage.get(key.into_bytes()) {
            Ok(Some(value)) => Ok(Some(from_utf8(value.as_ref()).unwrap().to_string())),
            Ok(None) => Ok(None),
            Err(err) => Err(KvError::KeyNotFound),
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        match &self.storage.get(key.as_bytes()) {
            Ok(Some(_)) => {
                &self.storage.remove(key.into_bytes());
                Ok(())
            }
            _ => Err(KvError::KeyNotFound)
        }
    }

    fn open(path: &Path) -> Result<SledKvsEngine> {
        let full_path = path.join(FILENAME);
        if !full_path.exists() {
            let storage = Db::open(path).unwrap();
            return Ok(SledKvsEngine { storage });
        } else {
            let first_10_bytes_of_sled = &[255, 186, 199, 15, 255, 255, 255, 255, 255, 255];
            let mut file = OpenOptions::new().read(true).open(full_path).unwrap();
            let mut first_bytes = [0; 10];
            file.read(&mut first_bytes);
            if !first_bytes.starts_with(first_10_bytes_of_sled) {
                return Err(KvError::IoError("Unable to open!".to_owned()));
            }
        }

        let storage = Db::open(path).unwrap();
        return Ok(SledKvsEngine { storage });
    }
}
