use std::convert::From;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TicketPriority {
    Normal,
    High,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriorityQueueTicket {
    code: usize,
    priority: TicketPriority,
}

impl PriorityQueueTicket {
    pub fn new(code: usize, priority: TicketPriority) -> Self {
        Self { code, priority }
    }

    pub fn code(&self) -> usize {
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
    next_ticket_number: usize,
}

impl PriorityQueue {
    pub fn new() -> Self {
        Self {
            high_priority_queue: Vec::new(),
            normal_priority_queue: Vec::new(),
            next_ticket_number: 1,
        }
    }

    pub fn enqueue(&mut self, ticket_priority: TicketPriority) -> Result<()> {
        let ticket = PriorityQueueTicket::new(self.next_ticket_number, ticket_priority);
        match ticket.priority {
            TicketPriority::Normal => {
                self.normal_priority_queue.push(ticket);
            }
            TicketPriority::High => {
                self.high_priority_queue.push(ticket);
            }
        }

        self.next_ticket_number += 1;

        Ok(())
    }

    pub fn get_high_priority_queue(&self) -> Vec<usize> {
        self.high_priority_queue
            .iter()
            .map(|ticket| ticket.code)
            .collect()
    }

    pub fn get_normal_priority_queue(&self) -> Vec<usize> {
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

    pub fn is_empty(&self) -> bool {
        self.high_priority_queue.is_empty() && self.normal_priority_queue.is_empty()
    }
}

impl Default for PriorityQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<PriorityQueueTicket>> for PriorityQueue {
    fn from(value: Vec<PriorityQueueTicket>) -> Self {
        let mut queue = PriorityQueue::new();
        let queue_len = value.len();

        queue.next_ticket_number = queue_len + 1;

        value.into_iter().for_each(|ticket| match ticket.priority {
            TicketPriority::High => queue.high_priority_queue.push(ticket),
            TicketPriority::Normal => queue.normal_priority_queue.push(ticket),
        });

        queue
    }
}
