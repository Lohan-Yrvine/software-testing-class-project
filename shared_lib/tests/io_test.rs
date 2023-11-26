use shared_lib::io_toolkit::InputHandler;
use shared_lib::priority_queue::TicketPriority;

#[test]
fn normal_priority_ticket_selection() {
    let input_mock = "2\n".as_bytes();
    let output_mock = Vec::with_capacity(0);
    match InputHandler::select_ticket_priority(input_mock, output_mock) {
        Some(TicketPriority::Normal) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn high_priority_ticket_selection() {
    let input_mock = "1\n".as_bytes();
    let output_mock = Vec::with_capacity(0);
    match InputHandler::select_ticket_priority(input_mock, output_mock) {
        Some(TicketPriority::High) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn wrong_priority_ticket_selection() {
    let input_mocks: Vec<&[u8]> = vec![b"4\n", b"5  \n", b"   6\n", b"\n", b"error\n", b"wrong\n"];
    for input in input_mocks.into_iter() {
        match InputHandler::select_ticket_priority(input, Vec::with_capacity(0)) {
            None => assert!(true),
            _ => assert!(false),
        }
    }
}
