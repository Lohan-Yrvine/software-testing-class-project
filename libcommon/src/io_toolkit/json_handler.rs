use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;

use anyhow::Result;
use serde::{de, Serialize};

pub struct JsonHandler;

impl JsonHandler {
    pub fn save_as_json<T>(path: &str, buff: &Vec<T>) -> Result<()>
    where
        T: Serialize,
    {
        if Path::new(path).exists() {
            fs::remove_file(path)?;
        }

        let serialized = serde_json::to_string_pretty(buff)?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    pub fn read_from_json<T>(path: &str) -> Result<Vec<T>>
    where
        T: de::DeserializeOwned,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let deserialized = serde_json::from_reader(reader)?;

        Ok(deserialized)
    }
}
