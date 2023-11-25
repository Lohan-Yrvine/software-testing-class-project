use std::io;

use sos_dentes::client::ClientManager;
use sos_dentes::input_handler::{InputHandler, OperationMode};

fn main() {
    const CLIENT_QUEUE_FILE_PATH: &'static str = "client_queue.json";

    let op_mode = InputHandler::select_operation_mode(io::stdin().lock(), io::stdout());
    loop {
        match op_mode {
            Some(OperationMode::Client) => ClientManager::start(CLIENT_QUEUE_FILE_PATH),
            Some(OperationMode::Service) => todo!(),
            Some(OperationMode::Dentist) => todo!(),
            None => println!("Modo INVÁLIDO. Por favor, insira novamente.\n"),
        }
    }
}
