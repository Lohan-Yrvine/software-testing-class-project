use std::convert::From;

use serde::{Deserialize, Serialize};

pub trait Priority {
    fn priority(&self) -> TicketPriority;
}

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
}

impl Priority for PriorityQueueTicket {
    fn priority(&self) -> TicketPriority {
        self.priority
    }
}

// not implementing with binary heap because of lack of time
pub struct PriorityQueue<T: Priority> {
    high_priority_queue: Vec<T>,
    normal_priority_queue: Vec<T>,
}

impl<T> PriorityQueue<T>
where
    T: Priority,
{
    pub fn new() -> Self {
        Self {
            high_priority_queue: Vec::new(),
            normal_priority_queue: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, element: T) {
        match element.priority() {
            TicketPriority::Normal => {
                self.normal_priority_queue.push(element);
            }
            TicketPriority::High => {
                self.high_priority_queue.push(element);
            }
        }
    }

    pub fn high_priority_queue(&self) -> &Vec<T> {
        &self.high_priority_queue
    }

    pub fn normal_priority_queue(&self) -> &Vec<T> {
        &self.normal_priority_queue
    }

    pub fn queue(&self) -> Vec<&T> {
        let mut high_priority_tickets: Vec<&T> = self.high_priority_queue.iter().collect();
        high_priority_tickets.extend(&self.normal_priority_queue);
        high_priority_tickets
    }

    pub fn is_empty(&self) -> bool {
        self.high_priority_queue.is_empty() && self.normal_priority_queue.is_empty()
    }
}

impl<T: Priority> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Priority> From<Vec<T>> for PriorityQueue<T> {
    fn from(value: Vec<T>) -> Self {
        let mut queue = PriorityQueue::new();

        value
            .into_iter()
            .for_each(|element| match element.priority() {
                TicketPriority::High => queue.high_priority_queue.push(element),
                TicketPriority::Normal => queue.normal_priority_queue.push(element),
            });

        queue
    }
}
