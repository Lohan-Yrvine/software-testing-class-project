use anyhow::{anyhow, Result};
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

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn priority(&self) -> TicketPriority {
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

    pub fn enqueue(&mut self, ticket_priority: TicketPriority) -> Result<()> {
        if self.max_tickets_amount == self.next_ticket_number {
            return Err(anyhow!("Queue is full"));
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

        Ok(())
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
        let mut high_priority_tickets: Vec<&PriorityQueueTicket> =
            self.high_priority_queue.iter().collect();

        let normal_priority_tickets: Vec<&PriorityQueueTicket> =
            self.normal_priority_queue.iter().collect();

        high_priority_tickets.extend(normal_priority_tickets);
        high_priority_tickets
    }

    pub fn max_tickets_amount(&self) -> u8 {
        self.max_tickets_amount
    }
}

impl Default for PriorityQueue {
    fn default() -> Self {
        Self::new()
    }
}
