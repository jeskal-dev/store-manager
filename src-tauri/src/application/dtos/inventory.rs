use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::inventory::Inventory;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateInventoryInput {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Inventory code must be between 1 and 20 characters"
    ))]
    pub inventory_code: String,

    pub store_id: String,

    pub product_id: String,

    pub quantity: i32,

    pub price_local: String,

    pub min_stock: i32,

    #[validate(length(min = 1, message = "Status cannot be empty"))]
    pub status: String,

    pub active: bool,
}

impl CreateInventoryInput {
    pub fn parse_price_local(&self) -> anyhow::Result<Decimal> {
        Ok(self.price_local.parse::<Decimal>()?)
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInventoryInput {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Inventory code must be between 1 and 20 characters"
    ))]
    #[serde(default)]
    pub inventory_code: Option<String>,
    #[serde(default)]
    pub store_id: Option<String>,
    #[serde(default)]
    pub product_id: Option<String>,
    #[serde(default)]
    pub quantity: Option<i32>,
    #[serde(default)]
    pub price_local: Option<String>,
    #[serde(default)]
    pub min_stock: Option<i32>,
    #[validate(length(min = 1, message = "Status cannot be empty"))]
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
}

impl UpdateInventoryInput {
    pub fn parse_price_local(&self) -> anyhow::Result<Option<Decimal>> {
        match &self.price_local {
            Some(s) => Ok(Some(s.parse::<Decimal>()?)),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryOutput {
    pub id: String,
    pub inventory_code: String,
    pub store_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub price_local: String,
    pub min_stock: i32,
    pub status: String,
    pub last_updated: String,
    pub active: bool,
}

impl From<&Inventory> for InventoryOutput {
    fn from(inventory: &Inventory) -> Self {
        Self {
            id: inventory.id.to_string(),
            inventory_code: inventory.inventory_code.value().to_string(),
            store_id: inventory.store_id.to_string(),
            product_id: inventory.product_id.to_string(),
            quantity: inventory.quantity.value(),
            price_local: inventory.price_local.formatted(),
            min_stock: inventory.min_stock.value(),
            status: inventory.status.as_str().to_string(),
            last_updated: inventory.last_updated.to_rfc3339(),
            active: inventory.active,
        }
    }
}
