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
    io_handler.set_remove_file_on_exit_handler(
        queue_file_path.clone(),
        Some("\n\nPrograma encerrado.".to_string()),
    )?;

    let mut manager = PacientManager::new(io_handler, PriorityQueue::new(), queue_file_path);
    manager.start();
}
