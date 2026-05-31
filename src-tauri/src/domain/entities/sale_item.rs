use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::value_objects::common::{Money, Quantity};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaleItem {
    pub id: Uuid,
    pub sale_id: Uuid,
    pub inventory_id: Uuid,
    pub quantity: Quantity,
    pub unit_price: Money,
    pub subtotal: Money,
}

impl SaleItem {
    pub fn new(
        sale_id: Uuid,
        inventory_id: Uuid,
        quantity: i32,
        unit_price: Decimal,
    ) -> Result<Self> {
        let quantity_vo = Quantity::new(quantity)?;
        let unit_price_money = Money::new(unit_price)?;
        let subtotal = Money::new(unit_price * Decimal::from(quantity))?;

        let sale_item = Self {
            id: Uuid::new_v4(),
            sale_id,
            inventory_id,
            quantity: quantity_vo,
            unit_price: unit_price_money,
            subtotal,
        };

        Ok(sale_item)
    }

    pub fn restore(
        id: Uuid,
        sale_id: Uuid,
        inventory_id: Uuid,
        quantity: i32,
        unit_price: Decimal,
        subtotal: Decimal,
    ) -> Result<Self> {
        let sale_item = Self {
            id,
            sale_id,
            inventory_id,
            quantity: Quantity::new(quantity)?,
            unit_price: Money::new(unit_price)?,
            subtotal: Money::new(subtotal)?,
        };

        Ok(sale_item)
    }

    pub fn calculate_subtotal(&self) -> Decimal {
        self.unit_price.amount() * Decimal::from(self.quantity.value())
    }

    pub fn update_quantity(&mut self, new_quantity: i32) -> Result<()> {
        self.quantity = Quantity::new(new_quantity)?;
        self.subtotal = Money::new(self.calculate_subtotal())?;
        Ok(())
    }

    pub fn update_unit_price(&mut self, new_price: Decimal) -> Result<()> {
        self.unit_price = Money::new(new_price)?;
        self.subtotal = Money::new(self.calculate_subtotal())?;
        Ok(())
    }
}
