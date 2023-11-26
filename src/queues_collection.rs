use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TicketPriority {
    Normal,
    High,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueTicket {
    code: u8,
    priority: TicketPriority,
}

impl QueueTicket {
    pub fn new(code: u8, priority: TicketPriority) -> Self {
        Self { code, priority }
    }

    pub fn get_code(&self) -> u8 {
        self.code
    }

    pub fn get_priority(&self) -> TicketPriority {
        self.priority
    }
}

pub struct PacientQueue {
    high_priority_queue: VecDeque<QueueTicket>,
    normal_priority_queue: VecDeque<QueueTicket>,
    next_ticket_number: u8,
    file_path: &'static str,
}

impl PacientQueue {
    pub fn new(file_path: &'static str) -> Self {
        File::create(file_path).expect("Unable to create pacient queue file");

        Self {
            high_priority_queue: VecDeque::with_capacity(u8::MAX as usize),
            normal_priority_queue: VecDeque::with_capacity(u8::MAX as usize),
            next_ticket_number: 0,
            file_path,
        }
    }

    pub fn get_high_priority_queue(&self) -> Vec<u8> {
        self.high_priority_queue
            .iter()
            .map(|ticket| ticket.code)
            .collect()
    }

    pub fn get_normal_priority_queue(&self) -> Vec<u8> {
        self.normal_priority_queue
            .iter()
            .map(|ticket| ticket.code)
            .collect()
    }

    pub fn take_ticket(&mut self, ticket_priority: TicketPriority) -> Option<u8> {
        if self.get_total_tickets_amount() == u8::MAX {
            return None;
        }

        if self.next_ticket_number == u8::MAX {
            self.next_ticket_number = 0;
        }

        self.next_ticket_number += 1;

        let ticket = QueueTicket::new(self.next_ticket_number, ticket_priority);

        match ticket.priority {
            TicketPriority::Normal => {
                self.normal_priority_queue.push_back(ticket);
            }
            TicketPriority::High => {
                self.high_priority_queue.push_back(ticket);
            }
        }

        self.enqueue_in_file();
        Some(self.next_ticket_number)
    }

    fn get_total_tickets_amount(&self) -> u8 {
        (self.high_priority_queue.len() + self.normal_priority_queue.len()) as u8
    }

    // always creating a new file and writing everything again since the file
    // has small size
    fn enqueue_in_file(&mut self) {
        if Path::new(self.file_path).exists() {
            fs::remove_file(self.file_path).expect("Unable to delete pacient queue file");
        }

        let serialized_queue = serde_json::to_string_pretty(&self.get_tickets())
            .expect("Unable to serialize pacient queue");

        let mut file = File::create(self.file_path).expect("Unable to create pacient queue file");
        file.write_all(serialized_queue.as_bytes())
            .expect("Unable to write serialized queue in file");
    }

    fn get_tickets(&self) -> Vec<&QueueTicket> {
        let mut high_priority_tickets: Vec<&QueueTicket> = self
            .high_priority_queue
            .iter()
            .map(|ticket| ticket)
            .collect();

        let normal_priority_tickets: Vec<&QueueTicket> = self
            .normal_priority_queue
            .iter()
            .map(|ticket| ticket)
            .collect();

        high_priority_tickets.extend(normal_priority_tickets);
        high_priority_tickets
    }

    pub fn get_amount_people_ahead(&self, ticket_priority: TicketPriority) -> u8 {
        match ticket_priority {
            TicketPriority::Normal => self.get_total_tickets_amount() - 1,
            TicketPriority::High => self.high_priority_queue.len() as u8 - 1,
        }
    }

    pub fn get_queue(&self) -> Vec<u8> {
        self.get_tickets()
            .iter()
            .map(|&ticket| ticket.code)
            .collect()
    }
}

impl Drop for PacientQueue {
    fn drop(&mut self) {
        if Path::new(self.file_path).exists() {
            fs::remove_file(self.file_path).expect("Unable to delete pacient queue file");
        }
    }
}
