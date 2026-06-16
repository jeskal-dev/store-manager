use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    entities::inventory::Inventory,
    entities::inventory_movement::InventoryMovement,
    events::inventory::{InventoryMovementRecorded, LowStockDetected, StockOutDetected},
    value_objects::inventory::{InventoryStatus, MovementType},
};

pub struct MovementResult {
    pub movement: InventoryMovement,
    pub updated_inventory: Inventory,
    pub requires_restock: bool,
    pub is_out_of_stock: bool,
}

impl MovementResult {
    pub fn movement_recorded_event(&self) -> InventoryMovementRecorded {
        InventoryMovementRecorded::build_movement_event(&self.movement, &self.updated_inventory)
    }

    pub fn low_stock_event(&self) -> Option<LowStockDetected> {
        if !self.requires_restock {
            return None;
        }
        Some(LowStockDetected {
            inventory_id: self.updated_inventory.id,
            store_id: self.updated_inventory.store_id,
            product_id: self.updated_inventory.product_id,
            current_quantity: self.updated_inventory.quantity.value(),
            min_stock: self.updated_inventory.min_stock.value(),
            occurred_at: self.movement.date,
        })
    }

    pub fn stock_out_event(&self) -> Option<StockOutDetected> {
        if !self.is_out_of_stock {
            return None;
        }
        Some(StockOutDetected {
            inventory_id: self.updated_inventory.id,
            store_id: self.updated_inventory.store_id,
            product_id: self.updated_inventory.product_id,
            occurred_at: self.movement.date,
        })
    }
}

fn calculate(
    inventory: &Inventory,
    movement_type: MovementType,
    old_qty: i32,
    new_qty: i32,
    description: Option<String>,
    purchase_item_id: Option<Uuid>,
    sale_item_id: Option<Uuid>,
) -> Result<MovementResult> {
    let movement = InventoryMovement::new(
        inventory.id,
        movement_type,
        old_qty,
        new_qty,
        description,
        purchase_item_id,
        sale_item_id,
    )?;

    let mut updated = inventory.clone();
    updated.update_quantity(new_qty)?;
    updated.calculate_status();

    let requires_restock = updated.requires_restock();
    let is_out_of_stock = matches!(updated.status, InventoryStatus::OutOfStock);

    Ok(MovementResult {
        movement,
        updated_inventory: updated,
        requires_restock,
        is_out_of_stock,
    })
}

pub fn calculate_purchase(
    inventory: &Inventory,
    quantity: i32,
    description: Option<String>,
    purchase_item_id: Uuid,
) -> Result<MovementResult> {
    let old_qty = inventory.quantity.value();
    let new_qty = old_qty + quantity;

    calculate(
        inventory,
        MovementType::Purchase,
        old_qty,
        new_qty,
        description,
        Some(purchase_item_id),
        None,
    )
}

pub fn calculate_sale(
    inventory: &Inventory,
    quantity: i32,
    description: Option<String>,
    sale_item_id: Uuid,
) -> Result<MovementResult> {
    let old_qty = inventory.quantity.value();

    if old_qty < quantity {
        anyhow::bail!(
            "Insufficient stock: available {}, requested {}",
            old_qty,
            quantity
        );
    }

    let new_qty = old_qty - quantity;

    calculate(
        inventory,
        MovementType::Sale,
        old_qty,
        new_qty,
        description,
        None,
        Some(sale_item_id),
    )
}

pub fn calculate_restock(
    inventory: &Inventory,
    quantity: i32,
    description: Option<String>,
) -> Result<MovementResult> {
    let old_qty = inventory.quantity.value();
    let new_qty = old_qty + quantity;

    calculate(
        inventory,
        MovementType::Restock,
        old_qty,
        new_qty,
        description,
        None,
        None,
    )
}

pub fn calculate_shrinkage(
    inventory: &Inventory,
    quantity: i32,
    description: Option<String>,
) -> Result<MovementResult> {
    let old_qty = inventory.quantity.value();

    if quantity <= 0 {
        anyhow::bail!("Shrinkage quantity must be positive");
    }

    let new_qty = (old_qty - quantity).max(0);

    calculate(
        inventory,
        MovementType::Shrinkage,
        old_qty,
        new_qty,
        description,
        None,
        None,
    )
}
