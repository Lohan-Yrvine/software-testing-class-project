use std::fs::{self, File};
use std::io::{BufReader, Write};

use anyhow::Result;
use common_lib::json_handler::JsonHandler;

#[test]
fn save_file_as_json_test() -> Result<()> {
    let path = "save_file_as_json_test.json";
    let buff = vec![1, 2, 3, 4, 5];

    JsonHandler::save_as_json(path, &buff)?;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let deserialized: Vec<i32> = serde_json::from_reader(reader)?;

    assert_eq!(deserialized, buff);

    fs::remove_file(path)?;

    Ok(())
}

#[test]
fn read_json_file_test() -> Result<()> {
    let path = "read_json_file_test.json";
    let buff = vec![1, 2, 3, 4, 5];

    let serialized = serde_json::to_string_pretty(&buff)?;
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;

    let result: Vec<u8> = JsonHandler::read_from_json(path)?;
    assert_eq!(result, buff);

    fs::remove_file(path)?;

    Ok(())
}
