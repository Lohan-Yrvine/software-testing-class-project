use std::env;

use common_lib::io_handler::{DefaultIOHandler, IOHandler};
use common_lib::priority_queue::PriorityQueue;
use dotenv::dotenv;

use receptionist::service_manager::ServiceManager;

fn main() {
    dotenv().ok();

    let pacient_queue_file_path =
        env::var("PACIENT_QUEUE_FILE_PATH").expect("Unable to get enviroment variable");
    let dentist_queue_file_path =
        env::var("DENTIST_QUEUE_FILE_PATH").expect("Unable to get enviroment variable");

    let io_handler = IOHandler::default();
    io_handler
        .set_remove_file_on_exit_handler(pacient_queue_file_path.clone())
        .expect("Unable to set remove on exit handler");

    let mut manager = ServiceManager::new(
        io_handler,
        PriorityQueue::new(),
        pacient_queue_file_path,
        PriorityQueue::new(),
        dentist_queue_file_path,
    );
    manager.start();
}
