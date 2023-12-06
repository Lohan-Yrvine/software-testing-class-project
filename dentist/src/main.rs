use std::env;

use anyhow::Result;
use common::io_handler::{DefaultIOHandler, IOHandler};
use dentist::servecing::Servecing;
use dotenv::dotenv;

fn main() -> Result<()> {
    dotenv().ok();

    let dentist_queue_file_path = env::var("DENTIST_QUEUE_FILE_PATH")?;

    let io_handler = IOHandler::default();
    io_handler.set_remove_file_on_exit_handler(
        "PlaceHolder".to_string(),
        Some("\n\nPrograma encerrado.".to_string()),
    )?;

    let mut servecing = Servecing::new(io_handler, dentist_queue_file_path);
    servecing.start();
}
