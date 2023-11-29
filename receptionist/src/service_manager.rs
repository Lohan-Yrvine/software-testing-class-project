use std::io;

use common::io_handler::IOHandler;
use common::priority_queue::PriorityQueue;

enum OperationMode {
    AttendPacient,
    ProcessPayment,
}

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
        self.io_handler
            .write("Obrigado por trabalhar conosco na SOS Dentes!\n")
            .expect("Unable to write welcome message");

        loop {
            let operation_input = self.get_operation_input();

            match self.parse_operation_input(&operation_input) {
                Some(OperationMode::AttendPacient) => self.attend_pacient(),
                Some(OperationMode::ProcessPayment) => self.process_payment(),
                None => self
                    .io_handler
                    .write(
                        "\nModo de operação é INVÁLIDO.\n\
                        Por favor, insira novamente.\n",
                    )
                    .expect("Unable to write invalid operation input error message"),
            }
        }
    }

    fn get_operation_input(&mut self) -> String {
        self.io_handler
            .write(
                "\n[1] Atender paciente na fila de espera\n\
                [2] Receber pagamento e finalizar atendimento\n\
                \n\
                Insira o tipo de operação que deseja fazer: ",
            )
            .expect("Unable to write available operations");

        self.io_handler
            .read_line()
            .expect("Unable to read the operation selected")
    }

    fn parse_operation_input(&self, operation: &str) -> Option<OperationMode> {
        let trimmed = operation.trim();
        if trimmed == "1" {
            return Some(OperationMode::AttendPacient);
        } else if trimmed == "2" {
            return Some(OperationMode::ProcessPayment);
        }

        None
    }

    fn attend_pacient(&self) {
        todo!()
    }

    fn process_payment(&self) {
        todo!()
    }
}
