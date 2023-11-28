use std::io;

use shared_lib::io_handler::IOHandler;
use shared_lib::json_handler::JsonHandler;
use shared_lib::priority_queue::{PriorityQueue, TicketPriority};

pub struct ServiceManager<R, W> {
    io_handler: IOHandler<R, W>,
    pacient_queue: PriorityQueue,
    pacient_queue_path: String,
    dentist_queue: PriorityQueue,
    dentist_queue_path: String,
}

impl<R, W> ServiceManager<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    pub fn new(
        io_handler: IOHandler<R, W>,
        pacient_queue: PriorityQueue,
        pacient_queue_path: String,
        dentist_queue: PriorityQueue,
        dentist_queue_path: String,
    ) -> Self {
        Self {
            io_handler,
            pacient_queue,
            pacient_queue_path,
            dentist_queue,
            dentist_queue_path,
        }
    }

    pub fn start(&mut self) -> ! {
        loop {
            todo!()
        }
    }
}
