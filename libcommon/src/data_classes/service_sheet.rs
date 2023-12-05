use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::database::GetKeyAttribute;
use crate::datetime_parsing::parse_datetime_from_default_fmt;
use crate::pacient_account::Pacient;
use crate::priority_queue::{Priority, TicketPriority};

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceSheet {
    pacient: Pacient,
    reason: String,
    date: String,
}

impl ServiceSheet {
    pub fn new(pacient: Pacient, reason: String, datetime: DateTime<Local>) -> Self {
        Self {
            pacient,
            reason,
            date: parse_datetime_from_default_fmt(datetime),
        }
    }

    pub fn pacient(&self) -> &str {
        self.pacient.name()
    }

    pub fn reason(&self) -> &str {
        self.reason.as_str()
    }

    pub fn date(&self) -> &str {
        self.date.as_str()
    }
}

impl GetKeyAttribute for ServiceSheet {
    fn get_key_attribute(&self) -> String {
        self.pacient.cpf().to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SheetWithPriority {
    service_sheet: ServiceSheet,
    priority: TicketPriority,
}

impl SheetWithPriority {
    pub fn new(service_sheet: ServiceSheet, priority: TicketPriority) -> Self {
        Self {
            service_sheet,
            priority,
        }
    }

    pub fn service_sheet(&self) -> &ServiceSheet {
        &self.service_sheet
    }

    pub fn priority(&self) -> TicketPriority {
        self.priority
    }
}

impl Priority for SheetWithPriority {
    fn priority(&self) -> TicketPriority {
        self.priority
    }
}
