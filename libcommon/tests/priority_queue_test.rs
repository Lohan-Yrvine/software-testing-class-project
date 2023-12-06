use rand;

use common::priority_queue::{PriorityQueue, PriorityQueueTicket, TicketPriority};

#[test]
fn taking_normal_priority_tickets_10_tickets() {
    let mut actual_queue = PriorityQueue::new();
    let mut expected_queue = Vec::new();
    let repetitions = 10;

    for code in 0..repetitions {
        let ticket = PriorityQueueTicket::new(code, TicketPriority::Normal);
        let _ = actual_queue.enqueue(ticket.clone());

        expected_queue.push(ticket);
    }

    assert_eq!(*actual_queue.normal_priority_queue(), expected_queue);
    assert!(actual_queue.high_priority_queue().is_empty());
}

#[test]
fn taking_normal_priority_tickets_255_tickets() {
    let mut actual_queue = PriorityQueue::new();
    let mut expected_queue = Vec::new();
    let repetitions = 255;

    for code in 0..repetitions {
        let ticket = PriorityQueueTicket::new(code, TicketPriority::Normal);
        let _ = actual_queue.enqueue(ticket.clone());

        expected_queue.push(ticket);
    }

    assert_eq!(*actual_queue.normal_priority_queue(), expected_queue);
    assert!(actual_queue.high_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_10_tickets() {
    let mut actual_queue = PriorityQueue::new();
    let mut expected_queue = Vec::new();
    let repetitions = 10;

    for code in 0..repetitions {
        let ticket = PriorityQueueTicket::new(code, TicketPriority::High);
        let _ = actual_queue.enqueue(ticket.clone());

        expected_queue.push(ticket);
    }

    assert_eq!(*actual_queue.high_priority_queue(), expected_queue);
    assert!(actual_queue.normal_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_255_tickets() {
    let mut actual_queue = PriorityQueue::new();
    let mut expected_queue = Vec::new();
    let repetitions = 255;

    for code in 0..repetitions {
        let ticket = PriorityQueueTicket::new(code, TicketPriority::High);
        let _ = actual_queue.enqueue(ticket.clone());

        expected_queue.push(ticket);
    }

    assert_eq!(*actual_queue.high_priority_queue(), expected_queue);
    assert!(actual_queue.normal_priority_queue().is_empty());
}

#[test]
fn taking_arbitrary_priority_tickets_5_tickets() {
    let mut actual_queue = PriorityQueue::new();

    actual_queue.enqueue(PriorityQueueTicket::new(1, TicketPriority::Normal));
    actual_queue.enqueue(PriorityQueueTicket::new(2, TicketPriority::Normal));
    actual_queue.enqueue(PriorityQueueTicket::new(3, TicketPriority::High));
    actual_queue.enqueue(PriorityQueueTicket::new(4, TicketPriority::Normal));
    actual_queue.enqueue(PriorityQueueTicket::new(5, TicketPriority::High));

    let expected_high_priority_queue = vec![
        PriorityQueueTicket::new(3, TicketPriority::High),
        PriorityQueueTicket::new(5, TicketPriority::High),
    ];
    let expected_normal_priority_queue = vec![
        PriorityQueueTicket::new(1, TicketPriority::Normal),
        PriorityQueueTicket::new(2, TicketPriority::Normal),
        PriorityQueueTicket::new(4, TicketPriority::Normal),
    ];

    assert_eq!(
        *actual_queue.high_priority_queue(),
        expected_high_priority_queue
    );
    assert_eq!(
        *actual_queue.normal_priority_queue(),
        expected_normal_priority_queue
    );

    let mut expected_queue = expected_high_priority_queue.clone();
    expected_queue.extend(expected_normal_priority_queue);
    let expected_queue: Vec<&PriorityQueueTicket> = expected_queue.iter().collect();

    assert_eq!(*actual_queue.queue(), expected_queue);
}

#[test]
fn taking_arbitrary_priority_tickets_255_tickets() {
    let mut actual_queue = PriorityQueue::new();
    let mut expected_high_priority_queue = Vec::new();
    let mut expected_normal_priority_queue = Vec::new();
    let repetitions = 255;

    for code in 1..=repetitions {
        let priority = if rand::random() {
            TicketPriority::Normal
        } else {
            TicketPriority::High
        };

        let ticket = PriorityQueueTicket::new(code, priority);
        actual_queue.enqueue(ticket.clone());

        match priority {
            TicketPriority::Normal => expected_normal_priority_queue.push(ticket.clone()),
            TicketPriority::High => expected_high_priority_queue.push(ticket.clone()),
        }
    }

    assert_eq!(
        *actual_queue.normal_priority_queue(),
        expected_normal_priority_queue
    );
    assert_eq!(
        *actual_queue.high_priority_queue(),
        expected_high_priority_queue
    );

    let mut expected_queue = expected_high_priority_queue.clone();
    expected_queue.extend(expected_normal_priority_queue);
    let expected_queue: Vec<&PriorityQueueTicket> = expected_queue.iter().collect();

    assert_eq!(actual_queue.queue(), expected_queue);
}

#[test]
fn convertion_from_vec_test() {
    let sample = vec![
        PriorityQueueTicket::new(1, TicketPriority::Normal),
        PriorityQueueTicket::new(2, TicketPriority::Normal),
        PriorityQueueTicket::new(3, TicketPriority::High),
        PriorityQueueTicket::new(4, TicketPriority::Normal),
        PriorityQueueTicket::new(5, TicketPriority::High),
    ];
    let mut expected_high_priority_queue = vec![
        PriorityQueueTicket::new(3, TicketPriority::High),
        PriorityQueueTicket::new(5, TicketPriority::High),
    ];
    let expected_normal_priority_queue = vec![
        PriorityQueueTicket::new(1, TicketPriority::Normal),
        PriorityQueueTicket::new(2, TicketPriority::Normal),
        PriorityQueueTicket::new(4, TicketPriority::Normal),
    ];

    let actual_queue = PriorityQueue::from(sample);

    assert_eq!(
        *actual_queue.high_priority_queue(),
        expected_high_priority_queue
    );
    assert_eq!(
        *actual_queue.normal_priority_queue(),
        expected_normal_priority_queue
    );

    expected_high_priority_queue.extend(expected_normal_priority_queue);
    let expected_queue: Vec<&PriorityQueueTicket> = expected_high_priority_queue.iter().collect();

    assert_eq!(*actual_queue.queue(), expected_queue);
}
