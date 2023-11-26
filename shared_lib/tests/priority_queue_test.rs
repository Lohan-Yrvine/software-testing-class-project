use rand;

use shared_lib::priority_queue::{PriorityQueue, PriorityQueueTicket, TicketPriority};

#[test]
fn taking_normal_priority_tickets_within_bounds() {
    let mut queue = PriorityQueue::new();
    let repetitions = 10;

    for _ in 0..repetitions {
        queue.take_ticket(TicketPriority::Normal);
    }

    let expected_queue: Vec<u8> = (1..=repetitions).collect();

    assert_eq!(queue.get_normal_priority_queue(), expected_queue);
    assert!(queue.get_high_priority_queue().is_empty());
}

#[test]
fn taking_normal_priority_tickets_out_of_bounds() {
    let mut queue = PriorityQueue::new();

    for _ in 0..300 {
        queue.take_ticket(TicketPriority::Normal);
    }

    let expected_queue: Vec<u8> = (1..=queue.get_max_tickets_amount()).collect();

    assert_eq!(queue.get_normal_priority_queue(), expected_queue);
    assert!(queue.get_high_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_within_bounds() {
    let mut queue = PriorityQueue::new();
    let repetitions = 10;

    for _ in 0..repetitions {
        queue.take_ticket(TicketPriority::High);
    }

    let expect_queue: Vec<u8> = (1..=repetitions).collect();

    assert_eq!(queue.get_high_priority_queue(), expect_queue);
    assert!(queue.get_normal_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_out_of_bounds() {
    let mut queue = PriorityQueue::new();

    for _ in 0..300 {
        queue.take_ticket(TicketPriority::High);
    }

    let expected_queue: Vec<u8> = (1..=queue.get_max_tickets_amount()).collect();

    assert_eq!(queue.get_high_priority_queue(), expected_queue);
    assert!(queue.get_normal_priority_queue().is_empty());
}

#[test]
fn taking_arbitrary_priority_tickets_within_bounds() {
    let mut queue = PriorityQueue::new();

    let ticket_code1 = queue.take_ticket(TicketPriority::Normal).unwrap();
    let ticket_code2 = queue.take_ticket(TicketPriority::Normal).unwrap();
    let ticket_code3 = queue.take_ticket(TicketPriority::High).unwrap();
    let ticket_code4 = queue.take_ticket(TicketPriority::Normal).unwrap();
    let ticket_code5 = queue.take_ticket(TicketPriority::High).unwrap();

    let expected_high_priority_queue = vec![ticket_code3, ticket_code5];
    let expected_normal_priority_queue = vec![ticket_code1, ticket_code2, ticket_code4];
    let expected_queue = vec![
        PriorityQueueTicket::new(ticket_code3, TicketPriority::High),
        PriorityQueueTicket::new(ticket_code5, TicketPriority::High),
        PriorityQueueTicket::new(ticket_code1, TicketPriority::Normal),
        PriorityQueueTicket::new(ticket_code2, TicketPriority::Normal),
        PriorityQueueTicket::new(ticket_code4, TicketPriority::Normal),
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

    for _ in 0..300 {
        let priority = if rand::random() {
            TicketPriority::Normal
        } else {
            TicketPriority::High
        };

        match queue.take_ticket(priority) {
            Some(code) => {
                aux_queue.push(PriorityQueueTicket::new(code, priority));
                match priority {
                    TicketPriority::Normal => expected_normal_priority_queue.push(code),
                    TicketPriority::High => expected_high_priority_queue.push(code),
                };
            }
            None => (),
        }
    }

    let mut expected_queue: Vec<&PriorityQueueTicket> = aux_queue
        .iter()
        .filter(|ticket| ticket.get_priority() == TicketPriority::High)
        .collect();
    expected_queue.extend(
        aux_queue
            .iter()
            .filter(|ticket| ticket.get_priority() == TicketPriority::Normal)
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
