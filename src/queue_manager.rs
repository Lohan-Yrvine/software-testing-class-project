use std::collections::VecDeque;
use std::io;

use crate::input_handler::select_ticket_priority;

#[derive(Clone)]
pub enum TicketPriority {
    Normal,
    High,
}

#[derive(Clone)]
pub struct QueueTicket {
    code: u8,
    priority: TicketPriority,
}

pub struct ClientQueueManager {
    priority_queue: VecDeque<QueueTicket>,
    normal_queue: VecDeque<QueueTicket>,
    next_ticket_number: u8,
    total_tickets: u8,
}

impl ClientQueueManager {
    pub fn new() -> Self {
        Self {
            priority_queue: VecDeque::with_capacity(u8::MAX as usize),
            normal_queue: VecDeque::with_capacity(u8::MAX as usize),
            next_ticket_number: 0,
            total_tickets: 0,
        }
    }

    // TODO: maybe find a way to gracefully terminate the program?
    pub fn start_routine(&mut self) -> ! {
        loop {
            let ticket_priority;
            match select_ticket_priority(io::stdin().lock(), io::stdout()) {
                Some(priority) => ticket_priority = priority,
                None => {
                    println!("Tipo de atendimento INVÁLIDO. Por favor, insira novamente.\n");
                    continue;
                }
            }

            let ticket_number = self.take_ticket(&ticket_priority);

            match ticket_number {
                Some(number) => {
                    let people_ahead_amount = self.calculate_people_ahead_amount(&ticket_priority);

                    println!(
                        "Seu número de chamada para atendimento é {}.\n\
                        Há um total de {} pessoas na sua frente.",
                        number, people_ahead_amount);
                }
                None => {
                    println!("Não há vagas no momento.");
                }
            }
            println!("Por favor, aguarde.\n");
        }
    }

    // TODO: save in a file
    fn take_ticket(&mut self, ticket_priority: &TicketPriority) -> Option<u8> {
        if self.total_tickets == u8::MAX {
            return None;
        }

        if self.next_ticket_number == u8::MAX {
            self.next_ticket_number = 0;
        }

        self.next_ticket_number += 1;

        let ticket = QueueTicket {
            code: self.next_ticket_number,
            priority: ticket_priority.clone(),
        };

        match ticket.priority {
            TicketPriority::Normal => {
                self.normal_queue.push_back(ticket);
            }
            TicketPriority::High => {
                self.priority_queue.push_back(ticket);
            }
        }

        Some(self.next_ticket_number)
    }

    fn calculate_people_ahead_amount(&self, ticket_priority: &TicketPriority) -> usize {
        match ticket_priority {
            TicketPriority::Normal => self.priority_queue.len() + self.normal_queue.len() - 1,
            TicketPriority::High => self.priority_queue.len() - 1,
        }
    }

    pub fn get_queue(&self) -> VecDeque<u8> {
        // copies the codes from the priority queue
        let mut result = self
            .priority_queue
            .iter()
            .cloned()
            .map(|element| element.code)
            .collect::<VecDeque<_>>();

        // extends with the codes from the normal queue
        result.extend(
            self.normal_queue
                .iter()
                .cloned()
                .map(|element| element.code)
                .collect::<VecDeque<_>>(),
        );

        result
    }

    pub fn dequeue(&mut self) -> Option<QueueTicket> {
        if self.priority_queue.is_empty() {
            return self.normal_queue.pop_front();
        }

        self.priority_queue.pop_front()
    }
}
