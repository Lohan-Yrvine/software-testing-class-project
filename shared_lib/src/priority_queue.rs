use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TicketPriority {
    Normal,
    High,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriorityQueueTicket {
    code: u8,
    priority: TicketPriority,
}

impl PriorityQueueTicket {
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

// not implementing with binary heap because of lack of time
pub struct PriorityQueue {
    high_priority_queue: Vec<PriorityQueueTicket>,
    normal_priority_queue: Vec<PriorityQueueTicket>,
    max_tickets_amount: u8,
    next_ticket_number: u8,
}

impl PriorityQueue {
    pub fn new() -> Self {
        let max_tickets_amount = 20usize;

        Self {
            high_priority_queue: Vec::with_capacity(max_tickets_amount),
            normal_priority_queue: Vec::with_capacity(max_tickets_amount),
            max_tickets_amount: max_tickets_amount as u8,
            next_ticket_number: 0,
        }
    }

    pub fn take_ticket(&mut self, ticket_priority: TicketPriority) -> Option<u8> {
        if self.max_tickets_amount == self.next_ticket_number {
            return None;
        }

        self.next_ticket_number += 1;

        let ticket = PriorityQueueTicket::new(self.next_ticket_number, ticket_priority);
        match ticket.priority {
            TicketPriority::Normal => {
                self.normal_priority_queue.push(ticket);
            }
            TicketPriority::High => {
                self.high_priority_queue.push(ticket);
            }
        }

        Some(self.next_ticket_number)
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

    pub fn get_queue(&self) -> Vec<&PriorityQueueTicket> {
        let mut high_priority_tickets: Vec<&PriorityQueueTicket> = self
            .high_priority_queue
            .iter()
            .map(|ticket| ticket)
            .collect();

        let normal_priority_tickets: Vec<&PriorityQueueTicket> = self
            .normal_priority_queue
            .iter()
            .map(|ticket| ticket)
            .collect();

        high_priority_tickets.extend(normal_priority_tickets);
        high_priority_tickets
    }

    pub fn get_max_tickets_amount(&self) -> u8 {
        self.max_tickets_amount
    }
}
