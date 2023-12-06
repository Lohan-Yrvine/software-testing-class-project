use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::database::GetKeyAttribute;

#[derive(Serialize, Deserialize)]
pub struct Appointment {
    pub cpf: String,
    pub date: String,
}

impl Appointment {
    pub fn new(cpf: String, date: String) -> Self {
        Self { cpf, date }
    }
}

impl GetKeyAttribute for Appointment {
    fn get_key_attribute(&self) -> String {
        self.cpf.to_string()
    }
}

impl Display for Appointment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CPF: {}", self.cpf)?;
        writeln!(f, "Data marcada: {}", self.date)
    }
}
