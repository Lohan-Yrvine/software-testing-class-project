use std::io;

use crate::queue_manager::TicketPriority;

#[derive(Debug)]
pub enum OperationMode {
    Client,
    Service,
    Dentist,
}

pub struct InputHandler;

impl InputHandler {
    pub fn select_operation_mode<R, W>(mut reader: R, mut writer: W) -> Option<OperationMode>
    where
        R: io::BufRead,
        W: io::Write,
    {
        write!(
            writer,
            "[1] Cliente\n\
            [2] Atendente\n\
            [3] Dentista\n\
            \n\
            Escolha o modo de operação: "
        )
        .expect("Unable to display available operation modes message");
        writer.flush().unwrap();

        let mut buff = String::new();
        reader
            .read_line(&mut buff)
            .expect("Unable to read operation mode selected");

        write!(writer, "\n").unwrap();

        if buff == "1\n" {
            return Some(OperationMode::Client);
        } else if buff == "2\n" {
            return Some(OperationMode::Service);
        } else if buff == "3\n" {
            return Some(OperationMode::Dentist);
        }

        None
    }

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

        write!(writer, "\n").unwrap();

        if buff == "1\n" {
            return Some(TicketPriority::High);
        } else if buff == "2\n" {
            return Some(TicketPriority::Normal);
        }

        None
    }
}
