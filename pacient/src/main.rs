use std::env;

use common_lib::io_handler::{DefaultIOHandler, IOHandler};
use common_lib::priority_queue::PriorityQueue;
use dotenv::dotenv;

use pacient::pacient_manager::PacientManager;

fn main() {
    dotenv().ok();

    let queue_file_path =
        env::var("PACIENT_QUEUE_FILE_PATH").expect("Unable to get enviroment variable");

    let io_handler = IOHandler::default();
    io_handler
        .set_remove_file_on_exit_handler(
            queue_file_path.clone(),
            Some("\n\nPrograma encerrado.".to_string()),
        )
        .expect("Unable to set remove on exit handler");

    let mut manager = PacientManager::new(io_handler, PriorityQueue::new(), queue_file_path);
    manager.start();
}
