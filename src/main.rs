use std::io;

use sos_dentes::client::start_client_mode;
use sos_dentes::input_handler::{select_operation_mode, OperationMode};

fn main() {
    loop {
        match select_operation_mode(io::stdin().lock(), io::stdout()) {
            Some(OperationMode::Client) => start_client_mode(),
            Some(OperationMode::Service) => todo!(),
            Some(OperationMode::Dentist) => todo!(),
            None => println!("Modo INVÁLIDO. Por favor, insira novamente.\n"),
        }
    }
}
