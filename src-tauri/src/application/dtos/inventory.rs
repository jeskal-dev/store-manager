use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInventoryInput {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Inventory code must be between 1 and 20 characters"
    ))]
    pub inventory_code: String,

    #[validate]
    pub store_id: String,

    #[validate]
    pub product_id: String,

    #[validate]
    pub quantity: i32,

    #[validate]
    pub price_local: f64,

    #[validate]
    pub min_stock: i32,

    #[validate(length(min = 1, message = "Status cannot be empty"))]
    pub status: String,

    pub active: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateInventoryInput {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Inventory code must be between 1 and 20 characters"
    ))]
    #[serde(default)]
    pub inventory_code: Option<String>,
    #[validate]
    #[serde(default)]
    pub store_id: Option<String>,
    #[validate]
    #[serde(default)]
    pub product_id: Option<String>,
    #[validate]
    #[serde(default)]
    pub quantity: Option<i32>,
    #[validate]
    #[serde(default)]
    pub price_local: Option<f64>,
    #[validate]
    #[serde(default)]
    pub min_stock: Option<i32>,
    #[validate(length(min = 1, message = "Status cannot be empty"))]
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
}