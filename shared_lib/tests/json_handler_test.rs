use std::fs::{self, File};
use std::io::{BufReader, Write};

use anyhow::Result;
use shared_lib::json_handler::JsonHandler;

#[test]
fn save_file_as_json_test() {
    let path = "save_file_as_json_test.json";
    let buff = vec![1, 2, 3, 4, 5];

    match JsonHandler::save_as_json(path, &buff) {
        Ok(_) => {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            let deserialized: Vec<i32> = serde_json::from_reader(reader).unwrap();

            assert_eq!(deserialized, buff);

            fs::remove_file(path).unwrap();
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn read_json_file_test() {
    let path = "read_json_file_test.json";
    let buff = vec![1, 2, 3, 4, 5];

    let serialized = serde_json::to_string_pretty(&buff).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();

    match JsonHandler::read_from_json(path) as Result<Vec<i32>> {
        Ok(result) => {
            assert_eq!(result, buff);

            fs::remove_file(path).unwrap();
        }
        Err(_) => assert!(false),
    }
}
