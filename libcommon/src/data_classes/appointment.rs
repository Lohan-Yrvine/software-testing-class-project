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
