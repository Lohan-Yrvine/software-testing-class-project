use std::env;
use std::fs;

use ctrlc;
use dotenv::dotenv;

use pacient::pacient_manager::PacientManager;

fn main() {
    dotenv().ok();

    ctrlc::set_handler(move || {
        println!("\n\nEncerrando programa...");
        fs::remove_file(
            env::var("PACIENT_QUEUE_FILE_PATH")
                .expect("Enviroment variable 'PACIENT_QUEUE_FILE_PATH' not set"),
        )
        .unwrap();
        println!("Programa encerrado.");
        std::process::exit(0);
    })
    .expect("Unable to set exit handler");

    PacientManager::start();
}
