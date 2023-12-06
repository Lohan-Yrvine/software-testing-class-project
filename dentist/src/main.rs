use std::env;

use anyhow::Result;
use common::io_handler::{DefaultIOHandler, IOHandler};
use dentist::serving::Serving;
use dotenv::dotenv;

fn main() -> Result<()> {
    dotenv().ok();

    let dentist_queue_file_path = env::var("DENTIST_QUEUE_FILE_PATH")?;
    let io_handler = IOHandler::default();

    ctrlc::set_handler(|| {
        println!("\n\nPrograma encerrado.");
        std::process::exit(0);
    })?;

    let mut servecing = Serving::new(io_handler, dentist_queue_file_path);
    servecing.start();
}
