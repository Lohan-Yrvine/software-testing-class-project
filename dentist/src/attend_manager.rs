use std::io;
use std::thread;
use std::time;

use common::io_handler::IOHandler;
use common::json_handler::JsonHandler;
use common::service_sheet::SheetWithPriority;

pub struct AttendManager<R, W> {
    io_handler: IOHandler<R, W>,
    queue_path: String,
}

impl<R, W> AttendManager<R, W>
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

            match self.call_next_pacient() {
                Some(sheet) => {
                    self.io_handler.write(sheet).unwrap();
                    self.io_handler.write("\nAtendendo paciente...\n").unwrap();
                    let dur = time::Duration::from_secs(3);
                    thread::sleep(dur);
                    self.io_handler.write("Atendimento finalizado\n").unwrap();
                }
                None => {
                    self.io_handler
                        .write("Não existem fichas no momento na fila.\n")
                        .unwrap();
                }
            }
        }
    }

    fn call_next_pacient(&self) -> Option<SheetWithPriority> {
        let mut sheets: Vec<SheetWithPriority> =
            JsonHandler::read_from_json(&self.queue_path).unwrap();

        if sheets.is_empty() {
            return None;
        }

        let result = sheets.remove(0);
        JsonHandler::save_as_json(&self.queue_path, &sheets).unwrap();
        Some(result)
    }
}
