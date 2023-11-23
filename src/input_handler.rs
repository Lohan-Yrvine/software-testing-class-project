use std::io;

#[derive(Debug)]
pub enum OperationMode {
    Client,
    Service,
    Dentist,
}

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
