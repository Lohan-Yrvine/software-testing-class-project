use chrono::{DateTime, Local};
use common::database::GetKeyAttributesValue;
use serde_derive::{Deserialize, Serialize};

use crate::datetime_parsing::parse_datetime_from_default_fmt;
use crate::pacient_account::Pacient;

#[derive(Serialize, Deserialize)]
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

impl GetKeyAttributesValue for ServiceSheet {
    fn get_key_attributes_value(&self) -> String {
        self.pacient.cpf().to_string()
    }
}
