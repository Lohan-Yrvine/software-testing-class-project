use chrono::{DateTime, Local};
use common::database::{Database, GetKeyAttributesValue};
use serde_derive::{Deserialize, Serialize};

use crate::datetime_parsing::parse_datetime_from_default_fmt;
use crate::service_sheet::ServiceSheet;

#[derive(Serialize, Deserialize)]
pub struct Address {
    street: String,
    neighborhood: String,
    city: String,
}

impl Address {
    pub fn new(street: String, neighborhood: String, city: String) -> Self {
        Self {
            street,
            neighborhood,
            city,
        }
    }

    pub fn street(&self) -> &str {
        &self.street
    }

    pub fn neighborhood(&self) -> &str {
        &self.neighborhood
    }

    pub fn city(&self) -> &str {
        &self.city
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pacient {
    name: String,
    cpf: String,
    phone_number: String,
    date_of_birth: String,
    address: Address,
    date_of_creation: String,
}

impl Pacient {
    pub fn new(
        name: String,
        cpf: String,
        phone_number: String,
        date_of_birth: String,
        address: Address,
        datetime_of_creation: DateTime<Local>,
    ) -> Self {
        Self {
            name,
            cpf,
            phone_number,
            date_of_birth,
            address,
            date_of_creation: parse_datetime_from_default_fmt(datetime_of_creation),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cpf(&self) -> &str {
        &self.cpf
    }

    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn date_of_birth(&self) -> &str {
        &self.date_of_birth
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn street(&self) -> &str {
        &self.address.street
    }

    pub fn neighborhood(&self) -> &str {
        &self.address.neighborhood
    }

    pub fn city(&self) -> &str {
        &self.address.city
    }

    pub fn date_of_creation(&self) -> &str {
        &self.date_of_creation
    }

    pub fn service_history(&self, database: &Database, key: &str) -> Vec<ServiceSheet> {
        database.query_vec(key).unwrap()
    }
}

impl GetKeyAttributesValue for Pacient {
    fn get_key_attributes_value(&self) -> String {
        self.cpf.to_string()
    }
}
