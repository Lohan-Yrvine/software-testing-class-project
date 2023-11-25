use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TicketPriority {
    Normal,
    High,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueTicket {
    code: u8,
    priority: TicketPriority,
}

impl QueueTicket {
    pub fn get_code(&self) -> u8 {
        self.code
    }

    pub fn get_priority(&self) -> TicketPriority {
        self.priority
    }
}

pub struct PacientQueue {
    priority_queue: VecDeque<QueueTicket>,
    normal_queue: VecDeque<QueueTicket>,
    next_ticket_number: u8,
    file_path: &'static str,
}

impl PacientQueue {
    pub fn new(file_path: &'static str) -> Self {
        File::create(file_path).expect("Unable to create queue file");

        Self {
            priority_queue: VecDeque::with_capacity(u8::MAX as usize),
            normal_queue: VecDeque::with_capacity(u8::MAX as usize),
            next_ticket_number: 0,
            file_path,
        }
    }

    pub fn get_high_priority_queue(&self) -> Vec<u8> {
        self.priority_queue
            .iter()
            .map(|ticket| ticket.code)
            .collect()
    }

    pub fn get_normal_priority_queue(&self) -> Vec<u8> {
        self.normal_queue.iter().map(|ticket| ticket.code).collect()
    }

    pub fn take_ticket(&mut self, ticket_priority: TicketPriority) -> Option<u8> {
        if self.get_total_tickets_amount() == u8::MAX {
            return None;
        }

        if self.next_ticket_number == u8::MAX {
            self.next_ticket_number = 0;
        }

        self.next_ticket_number += 1;

        let ticket = QueueTicket {
            code: self.next_ticket_number,
            priority: ticket_priority,
        };

        match ticket.priority {
            TicketPriority::Normal => {
                self.normal_queue.push_back(ticket);
            }
            TicketPriority::High => {
                self.priority_queue.push_back(ticket);
            }
        }

        self.enqueue_in_file();
        Some(self.next_ticket_number)
    }

    fn get_total_tickets_amount(&self) -> u8 {
        (self.priority_queue.len() + self.normal_queue.len()) as u8
    }

    fn enqueue_in_file(&mut self) {
        if Path::new(self.file_path).exists() {
            fs::remove_file(self.file_path).expect("Unable to delete queue file");
        }

        let serialized_queue = serde_json::to_string_pretty(&self.get_queue())
            .expect("Unable to serialize pacient queue");

        let mut file = File::create(self.file_path).expect("Unable to create queue file");
        file.write_all(serialized_queue.as_bytes())
            .expect("Unable to write serialized queue in file");
    }

    pub fn get_amount_people_ahead(&self, ticket_priority: TicketPriority) -> u8 {
        match ticket_priority {
            TicketPriority::Normal => self.get_total_tickets_amount() - 1,
            TicketPriority::High => self.priority_queue.len() as u8 - 1,
        }
    }

    pub fn get_queue(&self) -> Vec<u8> {
        // copies the codes from the priority queue
        let mut result: Vec<u8> = self
            .priority_queue
            .iter()
            .cloned()
            .map(|element| element.code)
            .collect();

        // extends with the codes from the normal queue
        result.extend(
            self.normal_queue
                .iter()
                .cloned()
                .map(|element| element.code)
                .collect::<Vec<_>>(),
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

impl Drop for PacientQueue {
    fn drop(&mut self) {
        if Path::new(self.file_path).exists() {
            fs::remove_file(self.file_path).expect("Unable to delete queue file");
        }
    }
}
