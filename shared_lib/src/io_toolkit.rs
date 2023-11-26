use std::io;

use crate::priority_queue::TicketPriority;

pub struct InputHandler;

impl InputHandler {
    pub fn select_ticket_priority<R, W>(mut reader: R, mut writer: W) -> Option<TicketPriority>
    where
        R: io::BufRead,
        W: io::Write,
    {
        write!(
            writer,
            "[1] Prioritário\n\
            [2] Normal\n\
            \n\
            Insira o tipo de atendimento para receber seu número de chamada: "
        )
        .expect("Unable to display available ticket priorities message");
        writer.flush().unwrap();

        let mut buff = String::new();
        reader
            .read_line(&mut buff)
            .expect("Unable to read ticket priority select");
        buff = buff.trim().to_string();

        writeln!(writer).unwrap();

        if buff == "1" {
            return Some(TicketPriority::High);
        } else if buff == "2" {
            return Some(TicketPriority::Normal);
        }

        None
    }
}
