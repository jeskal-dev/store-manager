use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::value_objects::common::{Money, Quantity};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseItem {
    pub id: Uuid,
    pub purchase_id: Uuid,
    pub inventory_id: Uuid,
    pub quantity: Quantity,
    pub unit_cost: Money,
    pub subtotal: Money,
}

impl PurchaseItem {
    pub fn new(
        purchase_id: Uuid,
        inventory_id: Uuid,
        quantity: i32,
        unit_cost: Decimal,
    ) -> Result<Self> {
        let quantity_vo = Quantity::new(quantity)?;
        let unit_cost_money = Money::new(unit_cost)?;
        let subtotal = Money::new(unit_cost * Decimal::from(quantity))?;

        let purchase_item = Self {
            id: Uuid::new_v4(),
            purchase_id,
            inventory_id,
            quantity: quantity_vo,
            unit_cost: unit_cost_money,
            subtotal,
        };

        Ok(purchase_item)
    }

    pub fn restore(
        id: Uuid,
        purchase_id: Uuid,
        inventory_id: Uuid,
        quantity: i32,
        unit_cost: Decimal,
        subtotal: Decimal,
    ) -> Result<Self> {
        let purchase_item = Self {
            id,
            purchase_id,
            inventory_id,
            quantity: Quantity::new(quantity)?,
            unit_cost: Money::new(unit_cost)?,
            subtotal: Money::new(subtotal)?,
        };

        Ok(purchase_item)
    }

    pub fn calculate_subtotal(&self) -> Decimal {
        self.unit_cost.amount() * Decimal::from(self.quantity.value())
    }

    pub fn update_quantity(&mut self, new_quantity: i32) -> Result<()> {
        self.quantity = Quantity::new(new_quantity)?;
        self.subtotal = Money::new(self.calculate_subtotal())?;
        Ok(())
    }

    pub fn update_unit_cost(&mut self, new_cost: Decimal) -> Result<()> {
        self.unit_cost = Money::new(new_cost)?;
        self.subtotal = Money::new(self.calculate_subtotal())?;
        Ok(())
    }
}
