use std::fs::{self, File};
use std::io::{BufReader, Write};

use anyhow::Result;
use common::appointment::Appointment;
use common::database::{Database, GetKeyAttribute};

#[test]
fn insert_test() -> Result<()> {
    let db_path = "insert_test.json.db".to_string();
    let db = Database::new(db_path.clone());

    let input = Appointment::new("123".to_string(), "456".to_string());

    db.insert(input.clone())?;

    let file = File::open(&db_path)?;
    let rdr = BufReader::new(file);
    let output: Vec<Appointment> = serde_json::from_reader(rdr)?;

    assert_eq!(output[0], input);

    fs::remove_file(db_path)?;

    Ok(())
}

#[test]
fn query_test() -> Result<()> {
    let db_path = "query_test.json.db".to_string();

    let input = vec![Appointment::new("123".to_string(), "456".to_string())];
    let serialized = serde_json::to_string_pretty(&input)?;
    let mut file = File::create(&db_path)?;
    file.write_all(serialized.as_bytes())?;

    let db = Database::new(db_path.clone());
    let output: Appointment = db.query("123")?;

    assert_eq!(output, input[0]);

    fs::remove_file(db_path)?;

    Ok(())
}

#[test]
fn query_vec_test() -> Result<()> {
    let db_path = "query_vec_test.json.db".to_string();

    let input = vec![
        Appointment::new("123".to_string(), "456".to_string()),
        Appointment::new("321".to_string(), "456".to_string()),
        Appointment::new("123".to_string(), "654".to_string()),
    ];
    let serialized = serde_json::to_string_pretty(&input)?;
    let mut file = File::create(&db_path)?;
    file.write_all(serialized.as_bytes())?;

    let db = Database::new(db_path.clone());
    let result: Vec<Appointment> = db.query_vec("123")?;

    assert_eq!(result, vec![input[0].clone(), input[2].clone()]);

    fs::remove_file(db_path)?;

    Ok(())
}

#[test]
fn query_all_test() -> Result<()> {
    let db_path = "query_all_test.json.db".to_string();

    let input = vec![
        Appointment::new("123".to_string(), "456".to_string()),
        Appointment::new("321".to_string(), "456".to_string()),
        Appointment::new("123".to_string(), "654".to_string()),
    ];
    let serialized = serde_json::to_string_pretty(&input)?;
    let mut file = File::create(&db_path)?;
    file.write_all(serialized.as_bytes())?;

    let db = Database::new(db_path.clone());
    let result: Vec<Appointment> = db.query_all()?;

    assert_eq!(result, input);

    fs::remove_file(db_path)?;

    Ok(())
}

#[test]
fn update_test() -> Result<()> {
    let db_path = "update_test.json.db".to_string();
    let db = Database::new(db_path.clone());

    let input = Appointment::new("123".to_string(), "456".to_string());
    db.insert(input.clone())?;

    let updated = Appointment::new("123".to_string(), "654".to_string());
    db.update(&input.get_key_attribute(), updated.clone())?;

    let file = File::open(&db_path)?;
    let rdr = BufReader::new(file);
    let output: Vec<Appointment> = serde_json::from_reader(rdr)?;

    assert_eq!(output[0], updated);

    fs::remove_file(db_path)?;

    Ok(())
}

#[test]
fn delete_test() -> Result<()> {
    let db_path = "delete_test.json.db".to_string();
    let db = Database::new(db_path.clone());

    let input = Appointment::new("123".to_string(), "456".to_string());
    db.insert(input.clone())?;

    let deleted: Appointment = db.delete(&input.get_key_attribute())?;

    let file = File::open(&db_path)?;
    let rdr = BufReader::new(file);
    let output: Vec<Appointment> = serde_json::from_reader(rdr)?;

    assert_eq!(deleted, input);
    assert_eq!(output, vec![]);

    fs::remove_file(db_path)?;

    Ok(())
}
