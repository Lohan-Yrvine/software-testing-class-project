use std::io;

use common::io_handler::IOHandler;

pub struct Servecing<R, W> {
    io_handler: IOHandler<R, W>,
    queue_path: String,
}

impl<R, W> Servecing<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    pub fn new(io_handler: IOHandler<R, W>, queue_path: String) -> Self {
        Self {
            io_handler,
            queue_path,
        }
    }

    pub fn start(&mut self) -> ! {
        loop {}
    }
}
