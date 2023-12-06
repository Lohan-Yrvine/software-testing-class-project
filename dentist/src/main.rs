use std::env;

use anyhow::Result;
use common::io_handler::{DefaultIOHandler, IOHandler};
use dentist::attend_manager::AttendManager;
use dotenv::dotenv;

fn main() -> Result<()> {
    dotenv().ok();

    let dentist_queue_file_path = env::var("DENTIST_QUEUE_FILE_PATH")?;
    let io_handler = IOHandler::default();

    let mut servecing = AttendManager::new(io_handler, dentist_queue_file_path);
    servecing.start();
}
