use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::inventory_movement::InventoryMovement;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPurchaseInput {
    pub inventory_id: String,
    /// Positive quantity added to stock
    pub quantity: i32,
    pub purchase_item_id: String,
    #[validate(length(max = 256))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSaleInput {
    pub inventory_id: String,
    /// Positive quantity removed from stock
    pub quantity: i32,
    pub sale_item_id: String,
    #[validate(length(max = 256))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRestockInput {
    pub inventory_id: String,
    /// Positive quantity added to stock
    pub quantity: i32,
    #[validate(length(max = 256))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterShrinkageInput {
    pub inventory_id: String,
    /// Positive quantity removed from stock
    pub quantity: i32,
    #[validate(length(max = 256))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryMovementOutput {
    pub movement_id: String,
    pub inventory_id: String,
    pub movement_type: String,
    pub old_quantity: i32,
    pub new_quantity: i32,
    pub requires_restock: bool,
    pub is_out_of_stock: bool,
    pub description: Option<String>,
}

impl InventoryMovementOutput {
    pub fn new(
        movement: &InventoryMovement,
        requires_restock: bool,
        is_out_of_stock: bool,
    ) -> Self {
        Self {
            movement_id: movement.id.to_string(),
            inventory_id: movement.inventory_id.to_string(),
            movement_type: movement.movement_type.as_str().to_string(),
            old_quantity: movement.old_quantity.value(),
            new_quantity: movement.new_quantity.value(),
            requires_restock,
            is_out_of_stock,
            description: movement.description.as_ref().map(|d| d.value().to_string()),
        }
    }
}
