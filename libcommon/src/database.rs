use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Result};
use serde::{de, Serialize};

use crate::json_handler::JsonHandler;

pub trait GetKeyAttribute {
    fn get_key_attribute(&self) -> String;
}

pub struct Database {
    path: String,
}

impl Database {
    pub fn new(path: String) -> Self {
        if !Path::new(&path).exists() {
            let mut file = File::create(&path).unwrap();
            file.write_all("[]".as_bytes()).unwrap();
        }

        Self { path }
    }

    pub fn insert<T>(&self, value: T) -> Result<()>
    where
        T: Serialize + de::DeserializeOwned + GetKeyAttribute,
    {
        let mut content: Vec<T> = JsonHandler::read_from_json(&self.path)?;
        content.push(value);

        JsonHandler::save_as_json(&self.path, &content)
    }

    pub fn query<T>(&self, key: &str) -> Result<T>
    where
        T: de::DeserializeOwned + GetKeyAttribute,
    {
        let content: Vec<T> = JsonHandler::read_from_json(&self.path)?;

        if let Some(element) = content
            .into_iter()
            .find(|element| element.get_key_attribute() == key)
        {
            return Ok(element);
        }

        Err(anyhow!("Element not found"))
    }

    pub fn query_vec<T>(&self, key: &str) -> Result<Vec<T>>
    where
        T: de::DeserializeOwned + GetKeyAttribute,
    {
        let content: Vec<T> = JsonHandler::read_from_json(&self.path)?;

        let result: Vec<T> = content
            .into_iter()
            .filter(|element| element.get_key_attribute() == key)
            .collect();

        if !result.is_empty() {
            return Ok(result);
        }

        Err(anyhow!("Element not found"))
    }

    pub fn query_all<T>(&self) -> Result<Vec<T>>
    where
        T: de::DeserializeOwned,
    {
        JsonHandler::read_from_json(&self.path)
    }

    pub fn update<T>(&self, key: &str, new_element: T) -> Result<()>
    where
        T: Serialize + de::DeserializeOwned + GetKeyAttribute,
    {
        let mut content: Vec<T> = JsonHandler::read_from_json(&self.path)?;

        if let Some(element) = content
            .iter_mut()
            .find(|element| element.get_key_attribute() == key)
        {
            *element = new_element;

            JsonHandler::save_as_json(&self.path, &content)?;
        }

        Ok(())
    }

    pub fn delete<T>(&self, key: &str) -> Result<T>
    where
        T: Serialize + de::DeserializeOwned + GetKeyAttribute,
    {
        let mut content: Vec<T> = JsonHandler::read_from_json(&self.path)?;

        if let Some(idx) = content
            .iter()
            .position(|element| element.get_key_attribute() == key)
        {
            let removed = content.swap_remove(idx);

            JsonHandler::save_as_json(&self.path, &content)?;

            return Ok(removed);
        }

        Err(anyhow!("Element not found"))
    }
}
