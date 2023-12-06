use std::io;

use chrono::Local;
use common::appointment::Appointment;
use common::database::{Database, GetKeyAttribute};
use common::io_handler::IOHandler;
use common::json_handler::JsonHandler;
use common::pacient_account::{Address, Pacient};
use common::priority_queue::{Priority, PriorityQueue, PriorityQueueTicket, TicketPriority};
use common::service_sheet::{ServiceSheet, SheetWithPriority};

enum OperationMode {
    AttendPacient,
    ProcessPayment,
    ManageAppointment,
    GetTodaysPayments,
}

pub struct ServiceManager<R, W> {
    io_handler: IOHandler<R, W>,
    pacient_queue_path: String,
    dentist_queue_path: String,
    pacient_accounts: Database,
    service_sheets_history: Database,
    appointment_schedule: Database,
    payments_of_the_day: Vec<usize>,
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
    ) -> Self {
        Self {
            io_handler,
            pacient_queue_path,
            dentist_queue_path,
            pacient_accounts,
            service_sheets_history,
            appointment_schedule,
            payments_of_the_day: Vec::new(),
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
                OperationMode::ManageAppointment => self.manage_appointments(),
                OperationMode::GetTodaysPayments => self.get_todays_payments(),
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
            OperationMode::AttendPacient
        } else if trimmed == "2" {
            OperationMode::ProcessPayment
        } else if trimmed == "3" {
            OperationMode::ManageAppointment
        } else {
            OperationMode::GetTodaysPayments
        }
    }

    fn attend_pacient(&mut self) {
        let ticket = self.get_next_pacient();
        self.io_handler
            .write(&format!("Código do próximo paciente: {}\n", ticket.code()))
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
            let account = self.get_pacient_account();
            self.check_pacient_data(account)
        } else {
            let key = self.create_pacient_account();
            self.pacient_accounts.query(&key).unwrap()
        };

        let sheet = self.create_service_sheet(pacient);
        self.enqueue_pacient_in_dentist_queue(sheet, ticket.priority());
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

        self.pacient_accounts.query(cpf.trim()).unwrap()
    }

    fn check_pacient_data(&mut self, mut pacient: Pacient) -> Pacient {
        self.io_handler.write("Dados do paciente:\n").unwrap();
        self.io_handler.write(&pacient).unwrap();

        self.io_handler
            .write(
                "\n[1] Sim\n\
                [2] Não\n\
                \n\
                Algum dos campos está errado? ",
            )
            .unwrap();
        let wrong_field = self.io_handler.read_line().unwrap();

        if wrong_field.trim() == "1" {
            self.update_pacient_account(&mut pacient);
        }

        pacient
    }

    // FIXME: the next two functions are extremely ugly
    fn update_pacient_account(&mut self, pacient: &mut Pacient) {
        self.io_handler
            .write("Deixe o campo vazio em caso de não alteração\n")
            .unwrap();

        self.io_handler.write("Número de celular: ").unwrap();
        let phone_number = self.io_handler.read_line().unwrap();
        if !phone_number.trim().is_empty() {
            pacient.set_phone_number(phone_number.trim().to_string());
        }

        self.io_handler.write("Rua: ").unwrap();
        let street = self.io_handler.read_line().unwrap();
        if !street.trim().is_empty() {
            pacient.set_street(street.trim().to_string());
        }

        self.io_handler.write("Bairro: ").unwrap();
        let neighborhood = self.io_handler.read_line().unwrap();
        if !neighborhood.trim().is_empty() {
            pacient.set_neighborhood(neighborhood.trim().to_string());
        }

        self.io_handler.write("Cidade: ").unwrap();
        let city = self.io_handler.read_line().unwrap();
        if !city.trim().is_empty() {
            pacient.set_city(city.trim().to_string());
        }

        self.pacient_accounts
            .update(&pacient.get_key_attribute(), pacient.clone())
            .unwrap();
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

    fn create_service_sheet(&mut self, pacient: Pacient) -> ServiceSheet {
        self.io_handler
            .write("\nCriando ficha de atendimento...\n")
            .unwrap();

        self.io_handler
            .write("Insira o motivo do atendimento: ")
            .unwrap();
        let mut reason = self.io_handler.read_line().unwrap();
        reason = reason.trim().to_string();

        let sheet = ServiceSheet::new(pacient, reason, Local::now());

        self.service_sheets_history.insert(sheet.clone()).unwrap();

        sheet
    }

    fn enqueue_pacient_in_dentist_queue(&self, sheet: ServiceSheet, priority: TicketPriority) {
        let mut dentist_queue;
        if let Ok(queue) = JsonHandler::read_from_json(&self.dentist_queue_path) {
            dentist_queue = PriorityQueue::from(queue);
        } else {
            dentist_queue = PriorityQueue::new()
        }
        dentist_queue.enqueue(SheetWithPriority::new(sheet, priority));
        JsonHandler::save_as_json(&self.dentist_queue_path, &dentist_queue.queue()).unwrap();
    }

    fn process_payment(&mut self) {
        self.io_handler
            .write("\nProcessando pagamento...\n")
            .unwrap();

        self.io_handler
            .write(
                "[1] Consulta de rotina - R$200\n\
                [2] Obturação - R$100\n\
                [3] Limpeza - R$50\n\
                \n\
                Qual foi o tipo de antedimento? ",
            )
            .unwrap();
        let procedure = self.io_handler.read_line().unwrap();

        if procedure.trim() == "1" {
            self.payments_of_the_day.push(200);
        } else if procedure.trim() == "2" {
            self.payments_of_the_day.push(100);
        } else {
            self.payments_of_the_day.push(50);
        }

        self.io_handler
            .write("[1] Pix\n[2] Cartão\n[3] Espécie\n\nInsira a forma de pagamento: ")
            .unwrap();
        let _ = self.io_handler.read_line().unwrap();

        self.io_handler.write("Pagamento efetuado\n").unwrap();
    }

    fn get_todays_payments(&mut self) {
        self.io_handler
            .write("\nRelatório de pagamentos do dia\n")
            .unwrap();
        self.io_handler
            .write(&format!(
                "Total arrecadado: R${}\n",
                self.payments_of_the_day.iter().sum::<usize>()
            ))
            .unwrap();
    }

    fn manage_appointments(&mut self) {
        self.io_handler
            .write(
                "[1] Marcar consulta\n\
                [2] Remarcar consulta\n\
                [3] Desmarcar consulta\n\
                [4] Mostrar consultas marcadas\n\
                \n\
                Insira a operação que deseja fazer: ",
            )
            .unwrap();
        let appointment_operation = self.io_handler.read_line().unwrap();

        if appointment_operation.trim() == "1" {
            self.make_appointment()
        } else if appointment_operation.trim() == "2" {
            self.update_appointment()
        } else if appointment_operation.trim() == "3" {
            self.delete_appointment()
        } else {
            self.show_appointments()
        }
    }

    fn make_appointment(&mut self) {
        self.io_handler.write("Marcando consulta...\n").unwrap();

        self.io_handler.write("CPF do paciente: ").unwrap();
        let cpf = self.io_handler.read_line().unwrap();

        self.io_handler.write("Data em dd-mm-aaaa: ").unwrap();
        let date = self.io_handler.read_line().unwrap();

        self.appointment_schedule
            .insert(Appointment::new(
                cpf.trim().to_string(),
                date.trim().to_string(),
            ))
            .unwrap();
    }

    fn update_appointment(&mut self) {
        self.io_handler.write("Remarcando consulta...\n").unwrap();

        self.io_handler.write("CPF do paciente: ").unwrap();
        let mut cpf = self.io_handler.read_line().unwrap();
        cpf = cpf.trim().to_string();

        self.io_handler
            .write("Insira a nova data (dd-mm-aaaa): ")
            .unwrap();
        let mut date = self.io_handler.read_line().unwrap();
        date = date.trim().to_string();

        let appointment = Appointment::new(cpf.clone(), date);

        self.appointment_schedule.update(&cpf, appointment).unwrap();
    }

    fn delete_appointment(&mut self) {
        self.io_handler.write("Desmarcando consulta...\n").unwrap();

        self.io_handler.write("CPF do paciente: ").unwrap();
        let cpf = self.io_handler.read_line().unwrap();

        let _: Appointment = self.appointment_schedule.delete(cpf.trim()).unwrap();
    }

    fn show_appointments(&mut self) {
        self.io_handler.write("Consultas marcadas").unwrap();

        let appointments: Vec<Appointment> = self.appointment_schedule.query_all().unwrap();
        appointments.into_iter().for_each(|appointment| {
            self.io_handler.write(appointment).unwrap();
            self.io_handler.write("\n").unwrap();
        })
    }
}
