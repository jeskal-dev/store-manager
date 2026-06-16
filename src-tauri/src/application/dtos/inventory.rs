use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

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
