use std::io;

use crate::queues_collection::TicketPriority;

#[derive(Debug)]
pub enum OperationMode {
    Pacient,
    Receptionist,
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
            "[1] Paciente\n\
            [2] Recepcionista\n\
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
        buff = buff.trim().to_string();

        writeln!(writer).unwrap();

        if buff == "1" {
            return Some(OperationMode::Pacient);
        } else if buff == "2" {
            return Some(OperationMode::Receptionist);
        } else if buff == "3" {
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
