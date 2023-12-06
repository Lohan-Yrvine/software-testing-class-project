use std::io::{self, Cursor};

use anyhow::Result;
use common::io_handler::IOHandler;

#[test]
fn read_line_test() -> Result<()> {
    let input = "Hello, World!\n";
    let cursor = Cursor::new(input);
    let mut io_handler = IOHandler::new(cursor, io::stdout());

    let result = io_handler.read_line()?;
    assert_eq!(result, input);

    Ok(())
}

#[test]
fn write_test() -> Result<()> {
    let input = "Hello, World\n";
    let cursor = Cursor::new(vec![]);
    let mut io_handler = IOHandler::new(io::stdin().lock(), cursor);

    io_handler.write(input)?;

    let result = String::from_utf8(io_handler.writer().clone().into_inner())?;
    assert_eq!(result, input);

    Ok(())
}
