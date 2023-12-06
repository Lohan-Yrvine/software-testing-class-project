use std::env;

use anyhow::Result;
use common::io_handler::{DefaultIOHandler, IOHandler};
use common::priority_queue::PriorityQueue;
use dotenv::dotenv;

use pacient::pacient_manager::PacientManager;

fn main() -> Result<()> {
    dotenv().ok();

    let queue_file_path = env::var("PACIENT_QUEUE_FILE_PATH")?;
    let io_handler = IOHandler::default();

    let mut manager = PacientManager::new(io_handler, PriorityQueue::new(), queue_file_path);
    manager.start();
    Ok(())
}
