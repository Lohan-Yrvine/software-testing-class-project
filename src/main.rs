use std::io;

use sos_dentes::client::start_client_mode;
use sos_dentes::input_handler::{select_operation_mode, OperationMode};

fn main() {
    const CLIENT_QUEUE_FILE_PATH: &'static str = "client_queue.json";

    loop {
        match select_operation_mode(io::stdin().lock(), io::stdout()) {
            Some(OperationMode::Client) => start_client_mode(CLIENT_QUEUE_FILE_PATH),
            Some(OperationMode::Service) => todo!(),
            Some(OperationMode::Dentist) => todo!(),
            None => println!("Modo INVÁLIDO. Por favor, insira novamente.\n"),
        }
    }
}
