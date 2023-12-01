use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use anyhow::Result;

use crate::json_handler::JsonHandler;

#[derive(Debug)]
pub enum DatabaseError {
    ElementNotFound,
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ElementNotFound => write!(f, "Element not found\n"),
        }
    }
}

impl Error for DatabaseError {}

pub struct Database<'a> {
    path: &'a str,
}

impl<'a> Database<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }

    pub fn insert<T>(&self, value: T) -> Result<()>
    where
        T: Into<HashMap<String, String>>,
    {
        let mut db_content: Vec<HashMap<String, String>> = JsonHandler::read_from_json(&self.path)?;
        db_content.push(value.into());

        JsonHandler::save_as_json(&self.path, &db_content)
    }

    pub fn query<T>(&self, key: &str) -> Result<T>
    where
        T: From<HashMap<String, String>>,
    {
        let db_content: Vec<HashMap<String, String>> = JsonHandler::read_from_json(&self.path)?;

        if let Some(hashmap) = db_content
            .into_iter()
            .find(|hashmap| hashmap.contains_key(key))
        {
            return Ok(hashmap.into());
        }

        Err(DatabaseError::ElementNotFound.into())
    }

    pub fn update<T>(&self, key: &str, element: T) -> Result<()>
    where
        T: Into<HashMap<String, String>>,
    {
        let mut db_content: Vec<HashMap<String, String>> = JsonHandler::read_from_json(&self.path)?;

        if let Some(hashmap) = db_content
            .iter_mut()
            .find(|hashmap| hashmap.contains_key(key))
        {
            *hashmap = element.into();

            JsonHandler::save_as_json(&self.path, &db_content)?;
        }

        Ok(())
    }

    pub fn delete<T>(&self, key: &str) -> Result<T>
    where
        T: From<HashMap<String, String>>,
    {
        let mut db_content: Vec<HashMap<String, String>> = JsonHandler::read_from_json(&self.path)?;

        if let Some(idx) = db_content
            .iter()
            .position(|hashmap| hashmap.contains_key(key))
        {
            let removed = db_content.swap_remove(idx);

            JsonHandler::save_as_json(&self.path, &db_content)?;

            return Ok(removed.into());
        }

        Err(DatabaseError::ElementNotFound.into())
    }
}
