use std::io;

use crate::input_handler::select_ticket_priority;
use crate::queue_manager::ClientQueueManager;

pub fn start_client_mode(client_queue_file_path: &'static str) {
    println!("Modo Cliente começando...\n");

    let mut queue = ClientQueueManager::new(client_queue_file_path);
    // TODO: maybe find a way to gracefully terminate the program?
    loop {
        let ticket_priority;
        match select_ticket_priority(io::stdin().lock(), io::stdout()) {
            Some(priority) => ticket_priority = priority,
            None => {
                println!("Tipo de atendimento INVÁLIDO. Por favor, insira novamente.\n");
                continue;
            }
        }

        let ticket_number = queue.take_ticket(ticket_priority.clone());

        match ticket_number {
            Some(number) => {
                let people_ahead_amount =
                    queue.calculate_people_ahead_amount(ticket_priority.clone());

                println!(
                    "Seu número de chamada para atendimento é {}.\n\
                        Há um total de {} pessoas na sua frente.",
                    number, people_ahead_amount
                );
            }
            None => {
                println!("Não há vagas no momento.");
            }
        }
        println!("Por favor, aguarde.\n");
    }
}
