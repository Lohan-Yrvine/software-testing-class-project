use std::env;

use dotenv::dotenv;
use shared_lib::IOToolkit;

use pacient::pacient_manager::PacientManager;

fn main() {
    dotenv().ok();

    let pacient_queue_file_path =
        env::var("PACIENT_QUEUE_FILE_PATH").expect("Unable to get enviroment variable");
    IOToolkit::remove_file_when_process_exits(pacient_queue_file_path.clone());

    let manager = PacientManager::new(pacient_queue_file_path);
    manager.start();
}
