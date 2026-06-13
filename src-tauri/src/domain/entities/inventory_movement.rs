use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::value_objects::common::{Quantity, TextValue};
use crate::domain::value_objects::inventory::MovementType;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InventoryMovement {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub movement_type: MovementType,
    pub old_quantity: Quantity,
    pub new_quantity: Quantity,
    pub date: DateTime<Utc>,
    pub description: Option<TextValue>,
    pub purchase_item_id: Option<Uuid>,
    pub sale_item_id: Option<Uuid>,
}

impl InventoryMovement {
    pub fn new(
        inventory_id: Uuid,
        movement_type: MovementType,
        old_quantity: i32,
        new_quantity: i32,
        description: Option<String>,
        purchase_item_id: Option<Uuid>,
        sale_item_id: Option<Uuid>,
    ) -> Result<Self> {
        let movement = Self {
            id: Uuid::new_v4(),
            inventory_id,
            movement_type,
            old_quantity: Quantity::new(old_quantity)?,
            new_quantity: Quantity::new(new_quantity)?,
            date: Utc::now(),
            description: description
                .map(|d| TextValue::new(d, 256))
                .transpose()?,
            purchase_item_id,
            sale_item_id,
        };
        Ok(movement)
    }

    pub fn restore(
        id: Uuid,
        inventory_id: Uuid,
        movement_type: MovementType,
        old_quantity: i32,
        new_quantity: i32,
        date: DateTime<Utc>,
        description: Option<String>,
        purchase_item_id: Option<Uuid>,
        sale_item_id: Option<Uuid>,
    ) -> Result<Self> {
        let movement = Self {
            id,
            inventory_id,
            movement_type,
            old_quantity: Quantity::new(old_quantity)?,
            new_quantity: Quantity::new(new_quantity)?,
            date,
            description: description
                .map(|d| TextValue::new(d, 256))
                .transpose()?,
            purchase_item_id,
            sale_item_id,
        };
        Ok(movement)
    }
}
