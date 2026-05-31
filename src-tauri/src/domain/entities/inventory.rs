use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::value_objects::{
    common::{Code, Money, Quantity},
    inventory::InventoryStatus,
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inventory {
    pub id: Uuid,
    pub inventory_code: Code,
    pub store_id: Uuid,
    pub product_id: Uuid,
    pub quantity: Quantity,
    pub price_local: Money,
    pub min_stock: Quantity,
    pub status: InventoryStatus,
    pub last_updated: DateTime<Utc>,
    pub active: bool,
}

impl Inventory {
    pub fn new(
        inventory_code: String,
        store_id: Uuid,
        product_id: Uuid,
        quantity: i32,
        price_local: Decimal,
        min_stock: i32,
        status: String,
        active: bool,
    ) -> Result<Self> {
        let inventory = Self {
            id: Uuid::new_v4(),
            inventory_code: Code::new(inventory_code)?,
            store_id,
            product_id,
            quantity: Quantity::new(quantity)?,
            price_local: Money::new(price_local)?,
            min_stock: Quantity::new(min_stock)?,
            status: InventoryStatus::new(status)?,
            last_updated: Utc::now(),
            active,
        };
        Ok(inventory)
    }

    pub fn restore(
        id: Uuid,
        inventory_code: String,
        store_id: Uuid,
        product_id: Uuid,
        quantity: i32,
        price_local: Decimal,
        min_stock: i32,
        status: String,
        last_updated: DateTime<Utc>,
        active: bool,
    ) -> Result<Self> {
        let inventory = Self {
            id,
            inventory_code: Code::new(inventory_code)?,
            store_id,
            product_id,
            quantity: Quantity::new(quantity)?,
            price_local: Money::new(price_local)?,
            min_stock: Quantity::new(min_stock)?,
            status: InventoryStatus::new(status)?,
            last_updated,
            active,
        };
        Ok(inventory)
    }

    pub fn update_quantity(&mut self, quantity: i32) -> Result<()> {
        self.quantity = Quantity::new(quantity)?;
        self.touch();
        Ok(())
    }

    pub fn update_price(&mut self, price: Decimal) -> Result<()> {
        self.price_local = Money::new(price)?;
        self.touch();
        Ok(())
    }

    pub fn update_status(&mut self, status: String) -> Result<()> {
        self.status = InventoryStatus::new(status)?;
        self.touch();
        Ok(())
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.touch();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.touch();
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn requires_restock(&self) -> bool {
        self.quantity.value() <= self.min_stock.value()
    }

    fn touch(&mut self) {
        self.last_updated = Utc::now();
    }
}
