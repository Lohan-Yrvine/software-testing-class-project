use std::io;

use chrono::Local;
use common::database::Database;
use common::io_handler::IOHandler;
use common::json_handler::JsonHandler;
use common::priority_queue::PriorityQueueTicket;

use crate::pacient_account::{Address, Pacient};
use crate::service_sheet::ServiceSheet;

enum OperationMode {
    AttendPacient,
    ProcessPayment,
    ManageAppointment,
}

pub struct ServiceManager<R, W> {
    io_handler: IOHandler<R, W>,
    pacient_queue_path: String,
    dentist_queue_path: String,
    pacient_accounts: Database,
    service_sheets_history: Database,
    appointment_schedule: Database,
    payment_records: Database,
}

impl<R, W> ServiceManager<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    pub fn new(
        io_handler: IOHandler<R, W>,
        pacient_queue_path: String,
        dentist_queue_path: String,
        pacient_accounts: Database,
        service_sheets_history: Database,
        appointment_schedule: Database,
        payment_records: Database,
    ) -> Self {
        Self {
            io_handler,
            pacient_queue_path,
            dentist_queue_path,
            pacient_accounts,
            service_sheets_history,
            appointment_schedule,
            payment_records,
        }
    }

    pub fn start(&mut self) -> ! {
        self.io_handler
            .write("Obrigado por trabalhar conosco na SOS Dentes!\n")
            .unwrap();

        loop {
            let operation_input = self.get_operation_input();

            match self.parse_operation_input(&operation_input) {
                OperationMode::AttendPacient => self.attend_pacient(),
                OperationMode::ProcessPayment => self.process_payment(),
                OperationMode::ManageAppointment => self.manage_appointment(),
            }
        }
    }

    fn get_operation_input(&mut self) -> String {
        self.io_handler
            .write(
                "\n[1] Atender paciente na fila de espera\n\
                [2] Receber pagamento e finalizar atendimento\n\
                [3] Gerenciar consultas\n\
                \n\
                Insira o tipo de operação que deseja fazer: ",
            )
            .unwrap();

        self.io_handler.read_line().unwrap()
    }

    fn parse_operation_input(&self, operation: &str) -> OperationMode {
        let trimmed = operation.trim();
        if trimmed == "1" {
            return OperationMode::AttendPacient;
        } else if trimmed == "2" {
            return OperationMode::ProcessPayment;
        } else {
            return OperationMode::ManageAppointment;
        }
    }

    fn attend_pacient(&mut self) {
        let next = self.get_next_pacient();
        self.io_handler
            .write(&format!("Código do próximo paciente: {}\n", next.code()))
            .unwrap();

        self.io_handler
            .write(
                "\n[1] Sim\n\
                [2] Não\n\
                \n\
                O paciente já possui conta? ",
            )
            .unwrap();
        let has_account = self.io_handler.read_line().unwrap();

        let pacient = if has_account.trim() == "1" {
            self.get_pacient_account()
        } else {
            let key = self.create_pacient_account();
            self.pacient_accounts.query(&key).unwrap()
        };

        self.create_service_sheet(pacient);
        self.enqueue_pacient_in_dentist_queue(next);
    }

    fn get_next_pacient(&self) -> PriorityQueueTicket {
        let mut queue: Vec<PriorityQueueTicket> =
            JsonHandler::read_from_json(&self.pacient_queue_path).unwrap();

        let result = queue.remove(0);

        JsonHandler::save_as_json(&self.pacient_queue_path, &queue).unwrap();

        result
    }

    fn get_pacient_account(&mut self) -> Pacient {
        self.io_handler
            .write("Realizando busca por paciente...\n")
            .unwrap();

        self.io_handler.write("Insira o CPF do paciente: ").unwrap();
        let cpf = self.io_handler.read_line().unwrap();

        self.pacient_accounts.query(&cpf).unwrap()

        // TODO: check pacient data to see if an update is necessary
    }

    fn create_pacient_account(&mut self) -> String {
        self.io_handler
            .write("\nCriando nova conta de paciente...\n")
            .unwrap();

        self.io_handler
            .write("Insira o nome do paciente: ")
            .unwrap();
        let mut name = self.io_handler.read_line().unwrap();
        name = name.trim().to_string();

        self.io_handler
            .write("\nInsira o cpf do paciente: ")
            .unwrap();
        let mut cpf = self.io_handler.read_line().unwrap();
        cpf = cpf.trim().to_string();
        let cpf_copy = cpf.to_string();

        self.io_handler
            .write("\nInsira um número para contato do paciente: ")
            .unwrap();
        let mut phone_number = self.io_handler.read_line().unwrap();
        phone_number = phone_number.trim().to_string();

        self.io_handler
            .write("\nInsira a data de nascimento do paciente: ")
            .unwrap();
        let mut date_of_birth = self.io_handler.read_line().unwrap();
        date_of_birth = date_of_birth.trim().to_string();

        self.io_handler
            .write("\nInsira a rua onde o paciente mora: ")
            .unwrap();
        let mut street = self.io_handler.read_line().unwrap();
        street = street.trim().to_string();

        self.io_handler
            .write("\nInsira o bairro onde o paciente mora: ")
            .unwrap();
        let mut neighborhood = self.io_handler.read_line().unwrap();
        neighborhood = neighborhood.trim().to_string();

        self.io_handler
            .write("\nInsira a cidade onde o paciente mora: ")
            .unwrap();
        let mut city = self.io_handler.read_line().unwrap();
        city = city.trim().to_string();

        let pacient_address = Address::new(street, neighborhood, city);

        let pacient = Pacient::new(
            name,
            cpf,
            phone_number,
            date_of_birth,
            pacient_address,
            Local::now(),
        );

        self.pacient_accounts.insert(pacient).unwrap();

        cpf_copy
    }

    fn create_service_sheet(&mut self, pacient: Pacient) {
        self.io_handler
            .write("Criando ficha de atendimento...\n")
            .unwrap();

        self.io_handler
            .write("Insira o motivo do atendimento: ")
            .unwrap();
        let mut reason = self.io_handler.read_line().unwrap();
        reason = reason.trim().to_string();

        let sheet = ServiceSheet::new(pacient, reason, Local::now());

        self.service_sheets_history.insert(sheet).unwrap();
    }

    fn enqueue_pacient_in_dentist_queue(&self, ticket: PriorityQueueTicket) {
        todo!()
    }

    fn process_payment(&self) {
        todo!()
    }

    fn manage_appointment(&self) {
        todo!()
    }
}
