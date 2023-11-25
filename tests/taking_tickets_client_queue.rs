use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;

use rand;
use serde_json;

use sos_dentes::queue_manager::{ClientQueue, TicketPriority};

#[inline]
fn get_test_file_content(path: &str) -> Vec<u8> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Unable to read from file")
}

#[test]
fn taking_normal_priority_tickets_within_bounds() {
    let test_file_path = "taking_normal_priority_tickets_within_bounds.json";
    let mut queue = ClientQueue::new(test_file_path);
    let repetitions = 10;

    let expected_queue: Vec<u8> = (0..repetitions)
        .filter_map(|_| queue.take_ticket(TicketPriority::Normal))
        .collect();

    let file_output = get_test_file_content(test_file_path);

    assert_eq!(queue.get_normal_priority_queue(), expected_queue);
    assert_eq!(file_output, expected_queue);
    assert_eq!(queue.get_normal_priority_queue().len(), repetitions);
    assert!(queue.get_high_priority_queue().is_empty());
}

#[test]
fn taking_normal_priority_tickets_out_of_bounds() {
    let test_file_path = "taking_normal_priority_tickets_out_of_bounds.json";
    let mut queue = ClientQueue::new(test_file_path);

    let expected_queue: Vec<u8> = (0..u8::MAX as usize + 1)
        .filter_map(|_| queue.take_ticket(TicketPriority::Normal))
        .collect();

    let file_output = get_test_file_content(test_file_path);

    assert_eq!(queue.get_normal_priority_queue(), expected_queue);
    assert_eq!(file_output, expected_queue);
    assert_eq!(queue.get_normal_priority_queue().len(), 255);
    assert!(queue.get_high_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_within_bounds() {
    let test_file_path = "taking_high_priority_tickets_within_bounds.json";
    let mut queue = ClientQueue::new(test_file_path);
    let repetitions = 10;

    let expected_queue: Vec<u8> = (0..10)
        .filter_map(|_| queue.take_ticket(TicketPriority::High))
        .collect();

    let file_output = get_test_file_content(test_file_path);

    assert_eq!(queue.get_high_priority_queue(), expected_queue);
    assert_eq!(file_output, expected_queue);
    assert_eq!(queue.get_high_priority_queue().len(), repetitions);
    assert!(queue.get_normal_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_out_of_bounds() {
    let test_file_path = "taking_high_priority_tickets_out_of_bounds.json";
    let mut queue = ClientQueue::new(test_file_path);

    let expected_queue: Vec<u8> = (0..u8::MAX as usize + 1)
        .filter_map(|_| queue.take_ticket(TicketPriority::High))
        .collect();

    let file_output = get_test_file_content(test_file_path);

    assert_eq!(queue.get_high_priority_queue(), expected_queue);
    assert_eq!(file_output, expected_queue);
    assert_eq!(queue.get_high_priority_queue().len(), 255);
    assert!(queue.get_normal_priority_queue().is_empty());
}

#[test]
fn taking_arbitrary_priority_tickets_within_bounds() {
    let test_file_path = "taking_arbitrary_priority_tickets_within_bounds.json";
    let mut queue = ClientQueue::new(test_file_path);

    let ticket1 = queue.take_ticket(TicketPriority::Normal).unwrap();
    let ticket2 = queue.take_ticket(TicketPriority::Normal).unwrap();
    let ticket3 = queue.take_ticket(TicketPriority::High).unwrap();
    let ticket4 = queue.take_ticket(TicketPriority::Normal).unwrap();
    let ticket5 = queue.take_ticket(TicketPriority::High).unwrap();

    let expected_queue = VecDeque::from([ticket3, ticket5, ticket1, ticket2, ticket4]);

    let file_output = get_test_file_content(test_file_path);

    assert_eq!(expected_queue, queue.get_queue());
    assert_eq!(file_output, queue.get_queue());
    assert_eq!(queue.get_normal_priority_queue().len(), 3);
    assert_eq!(queue.get_high_priority_queue().len(), 2);
}

#[test]
fn taking_arbitrary_priority_tickets_out_of_bounds() {
    let test_file_path = "taking_arbitrary_priority_tickets_out_of_bounds.json";
    let mut queue = ClientQueue::new(test_file_path);
    let mut expected_high_priority_queue = vec![];
    let mut expected_normal_priority_queue = vec![];

    // TODO: maybe find a better way to code this for loop?
    for _ in 0..u8::MAX as usize + 1 {
        let priority = if rand::random() {
            TicketPriority::Normal
        } else {
            TicketPriority::High
        };

        match queue.take_ticket(priority) {
            Some(code) => match priority {
                TicketPriority::Normal => expected_normal_priority_queue.push(code),
                TicketPriority::High => expected_high_priority_queue.push(code),
            },
            None => (),
        }
    }

    let file_output = get_test_file_content(test_file_path);

    assert_eq!(
        expected_normal_priority_queue,
        queue.get_normal_priority_queue()
    );
    assert_eq!(
        expected_high_priority_queue,
        queue.get_high_priority_queue()
    );

    expected_high_priority_queue.extend(expected_normal_priority_queue);
    let expected_queue = expected_high_priority_queue;

    assert_eq!(expected_queue, queue.get_queue());
    assert_eq!(file_output, queue.get_queue());
}
