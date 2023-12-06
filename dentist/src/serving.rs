use std::io;
use std::thread;
use std::time;

use common::io_handler::IOHandler;
use common::json_handler::JsonHandler;
use common::service_sheet::SheetWithPriority;

pub struct Serving<R, W> {
    io_handler: IOHandler<R, W>,
    queue_path: String,
}

impl<R, W> Serving<R, W>
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
        loop {
            self.io_handler
                .write("\nPressione [Enter] para chamar o próximo paciente\n")
                .unwrap();
            let _ = self.io_handler.read_line().unwrap();

            let sheet = self.call_next_pacient();

            self.io_handler.write(sheet).unwrap();
            self.io_handler.write("\nAtendendo paciente...\n").unwrap();
            let dur = time::Duration::from_secs(3);
            thread::sleep(dur);
            self.io_handler.write("Atendimento finalizado\n").unwrap();
        }
    }

    fn call_next_pacient(&self) -> SheetWithPriority {
        let mut sheets: Vec<SheetWithPriority> =
            JsonHandler::read_from_json(&self.queue_path).unwrap();
        let result = sheets.remove(0);
        JsonHandler::save_as_json(&self.queue_path, &sheets).unwrap();
        result
    }
}
