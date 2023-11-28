use std::io::{self, Write};

use shared_lib::priority_queue::{PriorityQueue, TicketPriority};
use shared_lib::IOToolkit;

pub struct PacientManager {
    queue_file_path: String,
}

impl PacientManager {
    pub fn new(queue_file_path: String) -> Self {
        Self { queue_file_path }
    }

    pub fn start(&self) -> ! {
        println!("Seja bem-vindo(a) à SOS Dentes!");

        let mut queue = PriorityQueue::new();
        loop {
            print!(
                "\n[1] Prioritário\n\
                [2] Normal\n\
                \n\
                Insira o tipo de atendimento para receber seu número de chamada: "
            );
            io::stdout().flush().unwrap();

            let mut ticket_priority = String::new();
            io::stdin()
                .read_line(&mut ticket_priority)
                .expect("Unable to read ticket priority");

            match self.handle_priority_input(&ticket_priority) {
                Some(priority) => self.handle_enqueue(&mut queue, priority),
                None => println!("\nTipo de atendimento INVÁLIDO. Por favor, insira novamente."),
            }
        }
    }

    fn handle_priority_input(&self, priority: &str) -> Option<TicketPriority> {
        let trimmed = priority.trim();
        if trimmed == "1" {
            return Some(TicketPriority::High);
        } else if trimmed == "2" {
            return Some(TicketPriority::Normal);
        }

        None
    }

    fn handle_enqueue(&self, queue: &mut PriorityQueue, priority: TicketPriority) {
        match queue.enqueue(priority) {
            Ok(()) => {
                IOToolkit::save_as_json(&self.queue_file_path, &queue.get_queue()).unwrap();
                println!(
                    "\nPedido de atendimento aceito.\n\
                    Você será chamado(a) quando for sua vez. Por favor, aguarde."
                );
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
