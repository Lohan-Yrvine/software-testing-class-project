use std::fs;
use std::io;
use std::path::Path;

use sos_dentes::client::ClientManager;
use sos_dentes::input_handler::{InputHandler, OperationMode};

const CLIENT_QUEUE_FILE_PATH: &'static str = "client_queue.json";

fn main() {
    ctrlc::set_handler(move || {
        println!("\n\nEncerrando programa...");
        println!("Apagando arquivos de filas...");
        remove_queue_files();
        println!("Programa encerrado.");
        std::process::exit(0);
    })
    .expect("Unable to set exit handler");

    let op_mode = InputHandler::select_operation_mode(io::stdin().lock(), io::stdout());
    match op_mode {
        Some(OperationMode::Client) => ClientManager::start(CLIENT_QUEUE_FILE_PATH),
        Some(OperationMode::Service) => todo!(),
        Some(OperationMode::Dentist) => todo!(),
        None => println!("Modo INVÁLIDO. Por favor, insira novamente.\n"),
    }
}

fn remove_queue_files() {
    let queue_files = vec![CLIENT_QUEUE_FILE_PATH];
    for file in queue_files {
        if Path::new(file).exists() {
            fs::remove_file(file).expect("Unable to delete client queue file");
        }
    }
}
