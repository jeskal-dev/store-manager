use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::value_objects::common::{Code, Name, PhoneNumber, TextValue};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub id: Uuid,
    pub store_code: Code,
    pub name: Name,
    pub address: TextValue,
    pub phone: Option<PhoneNumber>,
    pub active: bool,
}

impl Store {
    pub fn new(
        store_code: String,
        name: String,
        address: String,
        phone: Option<String>,
        active: bool,
    ) -> Result<Self> {
        let store = Self {
            id: Uuid::new_v4(),
            store_code: Code::new(store_code)?,
            name: Name::new(name)?,
            address: TextValue::new(address, 200)?,
            phone: match phone {
                Some(p) => Some(PhoneNumber::new(p)?),
                None => None,
            },
            active,
        };
        Ok(store)
    }

    pub fn restore(
        id: Uuid,
        store_code: String,
        name: String,
        address: String,
        phone: Option<String>,
        active: bool,
    ) -> Result<Self> {
        let store = Self {
            id,
            store_code: Code::new(store_code)?,
            name: Name::new(name)?,
            address: TextValue::new(address, 200)?,
            phone: match phone {
                Some(p) => Some(PhoneNumber::new(p)?),
                None => None,
            },
            active,
        };
        Ok(store)
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
