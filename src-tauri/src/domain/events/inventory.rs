use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{inventory::Inventory, inventory_movement::InventoryMovement};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryMovementRecorded {
    pub movement_id: Uuid,
    pub inventory_id: Uuid,
    pub store_id: Uuid,
    pub product_id: Uuid,
    pub movement_type: String,
    pub old_quantity: i32,
    pub new_quantity: i32,
    pub description: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

impl InventoryMovementRecorded {
    pub fn build_movement_event(movement: &InventoryMovement, inventory: &Inventory) -> Self {
        Self {
            movement_id: movement.id,
            inventory_id: movement.inventory_id,
            store_id: inventory.store_id,
            product_id: inventory.product_id,
            movement_type: movement.movement_type.as_str().to_string(),
            old_quantity: movement.old_quantity.value(),
            new_quantity: movement.new_quantity.value(),
            description: movement.description.as_ref().map(|d| d.value().to_string()),
            occurred_at: movement.date,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowStockDetected {
    pub inventory_id: Uuid,
    pub store_id: Uuid,
    pub product_id: Uuid,
    pub current_quantity: i32,
    pub min_stock: i32,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockOutDetected {
    pub inventory_id: Uuid,
    pub store_id: Uuid,
    pub product_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

// TODO: Add SaleCompleted event when sale module is implemented
// TODO: Add PurchaseCompleted event when purchase module is implemented
