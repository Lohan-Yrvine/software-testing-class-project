use rand;

use common::priority_queue::{PriorityQueue, PriorityQueueTicket, TicketPriority};

#[test]
fn taking_normal_priority_tickets_within_bounds() {
    let mut queue = PriorityQueue::new();
    let repetitions = 10;

    for _ in 0..repetitions {
        let _ = queue.enqueue(TicketPriority::Normal);
    }

    let expected_queue: Vec<u8> = (1..=repetitions).collect();

    assert_eq!(queue.get_normal_priority_queue(), expected_queue);
    assert!(queue.get_high_priority_queue().is_empty());
}

#[test]
fn taking_normal_priority_tickets_out_of_bounds() {
    let mut queue = PriorityQueue::new();

    for _ in 0..255 {
        let _ = queue.enqueue(TicketPriority::Normal);
    }

    let expected_queue: Vec<u8> = (1..=queue.max_tickets_amount()).collect();

    assert_eq!(queue.get_normal_priority_queue(), expected_queue);
    assert!(queue.get_high_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_within_bounds() {
    let mut queue = PriorityQueue::new();
    let repetitions = 10;

    for _ in 0..repetitions {
        let _ = queue.enqueue(TicketPriority::High);
    }

    let expect_queue: Vec<u8> = (1..=repetitions).collect();

    assert_eq!(queue.get_high_priority_queue(), expect_queue);
    assert!(queue.get_normal_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_out_of_bounds() {
    let mut queue = PriorityQueue::new();

    for _ in 0..255 {
        let _ = queue.enqueue(TicketPriority::High);
    }

    let expected_queue: Vec<u8> = (1..=queue.max_tickets_amount()).collect();

    assert_eq!(queue.get_high_priority_queue(), expected_queue);
    assert!(queue.get_normal_priority_queue().is_empty());
}

#[test]
fn taking_arbitrary_priority_tickets_within_bounds() {
    let mut queue = PriorityQueue::new();

    queue.enqueue(TicketPriority::Normal).unwrap();
    queue.enqueue(TicketPriority::Normal).unwrap();
    queue.enqueue(TicketPriority::High).unwrap();
    queue.enqueue(TicketPriority::Normal).unwrap();
    queue.enqueue(TicketPriority::High).unwrap();

    let expected_high_priority_queue: Vec<u8> = vec![3, 5];
    let expected_normal_priority_queue: Vec<u8> = vec![1, 2, 4];
    let expected_queue = vec![
        PriorityQueueTicket::new(3, TicketPriority::High),
        PriorityQueueTicket::new(5, TicketPriority::High),
        PriorityQueueTicket::new(1, TicketPriority::Normal),
        PriorityQueueTicket::new(2, TicketPriority::Normal),
        PriorityQueueTicket::new(4, TicketPriority::Normal),
    ];
    let expected_queue: Vec<&PriorityQueueTicket> =
        expected_queue.iter().map(|ticket| ticket).collect();

    assert_eq!(
        queue.get_high_priority_queue(),
        expected_high_priority_queue
    );
    assert_eq!(
        queue.get_normal_priority_queue(),
        expected_normal_priority_queue
    );
    assert_eq!(queue.get_queue(), expected_queue);
}

#[test]
fn taking_arbitrary_priority_tickets_out_of_bounds() {
    let mut queue = PriorityQueue::new();
    let mut expected_high_priority_queue = vec![];
    let mut expected_normal_priority_queue = vec![];
    let mut aux_queue = vec![];

    for code in 1..=255 {
        let priority = if rand::random() {
            TicketPriority::Normal
        } else {
            TicketPriority::High
        };

        match queue.enqueue(priority) {
            Ok(()) => {
                aux_queue.push(PriorityQueueTicket::new(code, priority));
                match priority {
                    TicketPriority::Normal => expected_normal_priority_queue.push(code),
                    TicketPriority::High => expected_high_priority_queue.push(code),
                };
            }
            Err(_) => (),
        }
    }

    let mut expected_queue: Vec<&PriorityQueueTicket> = aux_queue
        .iter()
        .filter(|ticket| ticket.priority() == TicketPriority::High)
        .collect();
    expected_queue.extend(
        aux_queue
            .iter()
            .filter(|ticket| ticket.priority() == TicketPriority::Normal)
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        queue.get_normal_priority_queue(),
        expected_normal_priority_queue
    );
    assert_eq!(
        queue.get_high_priority_queue(),
        expected_high_priority_queue
    );
    assert_eq!(queue.get_queue(), expected_queue);
}
