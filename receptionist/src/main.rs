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
    // FIXME: should not remove the dentist queue file, but doing it while
    // the dentist binary is not implemented
    io_handler
        .set_remove_file_on_exit_handler(
            dentist_queue_file_path.clone(),
            Some("\n\nPrograma encerrado.".to_string())
        )
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
