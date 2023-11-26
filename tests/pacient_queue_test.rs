use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use rand;
use serde_json;

use sos_dentes::queues_collection::{PacientQueue, TicketPriority};

#[test]
fn pacient_queue_file_creation() {
    let file_path = "creating_pacient_queue_file.json";
    let _queue = PacientQueue::new(file_path);
    assert!(Path::new(file_path).exists());
}

#[test]
fn pacient_queue_file_deletion() {
    let file_path = "removing_pacient_queue_file.json";
    // queue already drops here
    let _ = PacientQueue::new(file_path);
    assert!(!Path::new(file_path).exists());
}

#[inline]
fn parse_queue_file_content(path: &str) -> Vec<u8> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Unable to read from file")
}

#[test]
fn taking_normal_priority_tickets_within_bounds() {
    let file_path = "taking_normal_priority_tickets_within_bounds.json";
    let mut queue = PacientQueue::new(file_path);
    let repetitions = 10;

    for _ in 0..repetitions {
        queue.take_ticket(TicketPriority::Normal);
    }

    let expected_queue: Vec<u8> = (1..=repetitions).collect();

    let file_output = parse_queue_file_content(file_path);

    assert_eq!(queue.get_normal_priority_queue(), expected_queue);
    assert_eq!(file_output, expected_queue);
    assert!(queue.get_high_priority_queue().is_empty());
}

#[test]
fn taking_normal_priority_tickets_out_of_bounds() {
    let file_path = "taking_normal_priority_tickets_out_of_bounds.json";
    let mut queue = PacientQueue::new(file_path);

    for _ in 0..300 {
        queue.take_ticket(TicketPriority::Normal);
    }

    let expected_queue: Vec<u8> = (1..=255).collect();

    let file_output = parse_queue_file_content(file_path);

    assert_eq!(queue.get_normal_priority_queue(), expected_queue);
    assert_eq!(file_output, expected_queue);
    assert!(queue.get_high_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_within_bounds() {
    let file_path = "taking_high_priority_tickets_within_bounds.json";
    let mut queue = PacientQueue::new(file_path);
    let repetitions = 10;

    for _ in 0..repetitions {
        queue.take_ticket(TicketPriority::High);
    }

    let expect_queue: Vec<u8> = (1..=repetitions).collect();

    let file_output = parse_queue_file_content(file_path);

    assert_eq!(queue.get_high_priority_queue(), expect_queue);
    assert_eq!(file_output, expect_queue);
    assert!(queue.get_normal_priority_queue().is_empty());
}

#[test]
fn taking_high_priority_tickets_out_of_bounds() {
    let file_path = "taking_high_priority_tickets_out_of_bounds.json";
    let mut queue = PacientQueue::new(file_path);

    for _ in 0..300 {
        queue.take_ticket(TicketPriority::High);
    }

    let expected_queue: Vec<u8> = (1..=255).collect();

    let file_output = parse_queue_file_content(file_path);

    assert_eq!(queue.get_high_priority_queue(), expected_queue);
    assert_eq!(file_output, expected_queue);
    assert!(queue.get_normal_priority_queue().is_empty());
}

#[test]
fn taking_arbitrary_priority_tickets_within_bounds() {
    let file_path = "taking_arbitrary_priority_tickets_within_bounds.json";
    let mut queue = PacientQueue::new(file_path);

    let ticket1 = queue.take_ticket(TicketPriority::Normal).unwrap();
    let ticket2 = queue.take_ticket(TicketPriority::Normal).unwrap();
    let ticket3 = queue.take_ticket(TicketPriority::High).unwrap();
    let ticket4 = queue.take_ticket(TicketPriority::Normal).unwrap();
    let ticket5 = queue.take_ticket(TicketPriority::High).unwrap();

    let mut expected_high_priority_queue = vec![ticket3, ticket5];
    let expected_normal_priority_queue = vec![ticket1, ticket2, ticket4];

    let file_output = parse_queue_file_content(file_path);

    assert_eq!(
        queue.get_high_priority_queue(),
        expected_high_priority_queue
    );
    assert_eq!(
        queue.get_normal_priority_queue(),
        expected_normal_priority_queue
    );

    expected_high_priority_queue.extend(expected_normal_priority_queue);
    let expected_queue = expected_high_priority_queue;

    assert_eq!(queue.get_queue(), expected_queue);
    assert_eq!(file_output, expected_queue);
}

#[test]
fn taking_arbitrary_priority_tickets_out_of_bounds() {
    let file_path = "taking_arbitrary_priority_tickets_out_of_bounds.json";
    let mut queue = PacientQueue::new(file_path);
    let mut expected_high_priority_queue = vec![];
    let mut expected_normal_priority_queue = vec![];

    for _ in 0..300 {
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

    let file_output = parse_queue_file_content(file_path);

    assert_eq!(
        queue.get_normal_priority_queue(),
        expected_normal_priority_queue
    );
    assert_eq!(
        queue.get_high_priority_queue(),
        expected_high_priority_queue
    );

    expected_high_priority_queue.extend(expected_normal_priority_queue);
    let expected_queue = expected_high_priority_queue;

    assert_eq!(queue.get_queue(), expected_queue);
    assert_eq!(file_output, expected_queue);
}
