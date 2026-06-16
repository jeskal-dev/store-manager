use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::value_objects::common::{Code, Name, PhoneNumber, TextValue};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Supplier {
    pub id: Uuid,
    pub name: Name,
    pub supplier_code: Code,
    pub contact_name: Option<TextValue>,
    pub phone: Option<PhoneNumber>,
    pub address: Option<TextValue>,
    pub active: bool,
}

impl Supplier {
    pub fn new(
        name: String,
        supplier_code: String,
        contact_name: Option<String>,
        phone: Option<String>,
        address: Option<String>,
        active: bool,
    ) -> Result<Self> {
        let supplier = Self {
            id: Uuid::new_v4(),
            name: Name::new(name)?,
            supplier_code: Code::new(supplier_code)?,
            contact_name: match contact_name {
                Some(cn) => Some(TextValue::new(cn, 100)?),
                None => None,
            },
            phone: match phone {
                Some(p) => Some(PhoneNumber::new(p)?),
                None => None,
            },
            address: match address {
                Some(addr) => Some(TextValue::new(addr, 300)?),
                None => None,
            },
            active,
        };
        Ok(supplier)
    }

    pub fn restore(
        id: Uuid,
        name: String,
        supplier_code: String,
        contact_name: Option<String>,
        phone: Option<String>,
        address: Option<String>,
        active: bool,
    ) -> Result<Self> {
        let supplier = Self {
            id,
            name: Name::new(name)?,
            supplier_code: Code::new(supplier_code)?,
            contact_name: match contact_name {
                Some(cn) => Some(TextValue::new(cn, 100)?),
                None => None,
            },
            phone: match phone {
                Some(p) => Some(PhoneNumber::new(p)?),
                None => None,
            },
            address: match address {
                Some(addr) => Some(TextValue::new(addr, 200)?),
                None => None,
            },
            active,
        };
        Ok(supplier)
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
