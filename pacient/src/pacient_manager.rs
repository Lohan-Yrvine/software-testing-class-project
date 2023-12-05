use std::io;

use common::io_handler::IOHandler;
use common::json_handler::JsonHandler;
use common::priority_queue::{PriorityQueue, PriorityQueueTicket, TicketPriority};

pub struct PacientManager<R, W> {
    io_handler: IOHandler<R, W>,
    queue: PriorityQueue<PriorityQueueTicket>,
    queue_path: String,
    ticket_code: usize,
}

impl<R, W> PacientManager<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    pub fn new(
        io_handler: IOHandler<R, W>,
        queue: PriorityQueue<PriorityQueueTicket>,
        queue_path: String,
    ) -> Self {
        Self {
            io_handler,
            queue,
            queue_path,
            ticket_code: 1,
        }
    }

    pub fn start(&mut self) -> ! {
        self.io_handler
            .write("Seja bem-vindo(a) à SOS Dentes!\n")
            .unwrap();

        loop {
            let ticket_priority = self.get_ticket_priority_input();

            match self.parse_ticket_priority_input(&ticket_priority) {
                Some(priority) => self.handle_enqueue(priority),
                None => self
                    .io_handler
                    .write(
                        "\nTipo de atendimento INVÁLIDO.\n\
                        Por favor, insira novamente.\n",
                    )
                    .unwrap(),
            }
        }
    }

    fn get_ticket_priority_input(&mut self) -> String {
        self.io_handler
            .write(
                "\n[1] Prioritário\n\
                [2] Normal\n\
                \n\
                Insira o tipo de atendimento desejado\n\
                para entrar na fila de atendimento: ",
            )
            .unwrap();

        self.io_handler.read_line().unwrap()
    }

    fn parse_ticket_priority_input(&self, priority: &str) -> Option<TicketPriority> {
        let trimmed = priority.trim();
        if trimmed == "1" {
            return Some(TicketPriority::High);
        } else if trimmed == "2" {
            return Some(TicketPriority::Normal);
        }

        None
    }

    fn handle_enqueue(&mut self, priority: TicketPriority) {
        self.pull_file_updates();

        let ticket = PriorityQueueTicket::new(self.ticket_code, priority);
        self.queue.enqueue(ticket);

        JsonHandler::save_as_json(&self.queue_path, &self.queue.queue()).unwrap();

        self.ticket_code += 1;

        let accepted_service_msg = "\nPedido de atendimento aceito.\n\
                    Você será chamado(a) quando for sua vez.\nPor favor, aguarde.\n";

        self.io_handler.write(accepted_service_msg).unwrap();
    }

    fn pull_file_updates(&mut self) {
        if let Ok(queue) = JsonHandler::read_from_json(&self.queue_path) {
            self.queue = PriorityQueue::from(queue);
        }
    }
}
