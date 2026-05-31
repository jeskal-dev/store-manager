use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::value_objects::common::{Code, Money, Name};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub product_code: Code,
    pub name: Name,
    pub initial_price: Money,
    pub active: bool,
}

impl Product {
    pub fn new(product_code: String, name: String, price: Decimal, active: bool) -> Result<Self> {
        let product = Self {
            id: Uuid::new_v4(),
            product_code: Code::new(product_code)?,
            name: Name::new(name)?,
            initial_price: Money::new(price)?,
            active,
        };
        Ok(product)
    }

    pub fn restore(
        id: Uuid,
        product_code: String,
        name: String,
        price: Decimal,
        active: bool,
    ) -> Result<Self> {
        let product = Self {
            id,
            product_code: Code::new(product_code)?,
            name: Name::new(name)?,
            initial_price: Money::new(price)?,
            active,
        };
        Ok(product)
    }

    // Métodos de dominio sin validaciones repetidas
    pub fn update_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn update_name(&mut self, name: String) -> Result<()> {
        self.name = Name::new(name)?;
        Ok(())
    }

    pub fn update_price(&mut self, price: Decimal) -> Result<()> {
        self.initial_price = Money::new(price)?;
        Ok(())
    }

    pub fn update_product_code(&mut self, code: String) -> Result<()> {
        self.product_code = Code::new(code)?;
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_formatted_price(&self) -> String {
        self.initial_price.formatted()
    }
}
