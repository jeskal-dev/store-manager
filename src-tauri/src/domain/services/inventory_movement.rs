use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    entities::inventory::Inventory,
    entities::inventory_movement::InventoryMovement,
    value_objects::inventory::MovementType,
};

pub struct MovementResult {
    pub movement: InventoryMovement,
    pub updated_inventory: Inventory,
    pub requires_restock: bool,
    pub is_out_of_stock: bool,
}

pub fn calculate_purchase(
    inventory: &Inventory,
    quantity: i32,
    description: Option<String>,
    purchase_item_id: Uuid,
) -> Result<MovementResult> {
    let old_qty = inventory.quantity.value();
    let new_qty = old_qty + quantity;

    let movement = InventoryMovement::new(
        inventory.id,
        MovementType::Purchase,
        old_qty,
        new_qty,
        description,
        Some(purchase_item_id),
        None,
    )?;

    let mut updated = inventory.clone();
    updated.update_quantity(new_qty)?;
    updated.calculate_status();

    let requires_restock = updated.requires_restock();
    let is_out_of_stock = matches!(updated.status, crate::domain::value_objects::inventory::InventoryStatus::OutOfStock);

    Ok(MovementResult {
        movement,
        updated_inventory: updated,
        requires_restock,
        is_out_of_stock,
    })
}

pub fn calculate_sale(
    inventory: &Inventory,
    quantity: i32,
    description: Option<String>,
    sale_item_id: Uuid,
) -> Result<MovementResult> {
    let old_qty = inventory.quantity.value();

    if old_qty < quantity {
        anyhow::bail!("Insufficient stock: available {}, requested {}", old_qty, quantity);
    }

    let new_qty = old_qty - quantity;

    let movement = InventoryMovement::new(
        inventory.id,
        MovementType::Sale,
        old_qty,
        new_qty,
        description,
        None,
        Some(sale_item_id),
    )?;

    let mut updated = inventory.clone();
    updated.update_quantity(new_qty)?;
    updated.calculate_status();

    let requires_restock = updated.requires_restock();
    let is_out_of_stock = matches!(updated.status, crate::domain::value_objects::inventory::InventoryStatus::OutOfStock);

    Ok(MovementResult {
        movement,
        updated_inventory: updated,
        requires_restock,
        is_out_of_stock,
    })
}

pub fn calculate_restock(
    inventory: &Inventory,
    quantity: i32,
    description: Option<String>,
) -> Result<MovementResult> {
    let old_qty = inventory.quantity.value();
    let new_qty = old_qty + quantity;

    let movement = InventoryMovement::new(
        inventory.id,
        MovementType::Restock,
        old_qty,
        new_qty,
        description,
        None,
        None,
    )?;

    let mut updated = inventory.clone();
    updated.update_quantity(new_qty)?;
    updated.calculate_status();

    let requires_restock = updated.requires_restock();
    let is_out_of_stock = matches!(updated.status, crate::domain::value_objects::inventory::InventoryStatus::OutOfStock);

    Ok(MovementResult {
        movement,
        updated_inventory: updated,
        requires_restock,
        is_out_of_stock,
    })
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

    let movement = InventoryMovement::new(
        inventory.id,
        MovementType::Shrinkage,
        old_qty,
        new_qty,
        description,
        None,
        None,
    )?;

    let mut updated = inventory.clone();
    updated.update_quantity(new_qty)?;
    updated.calculate_status();

    let requires_restock = updated.requires_restock();
    let is_out_of_stock = matches!(updated.status, crate::domain::value_objects::inventory::InventoryStatus::OutOfStock);

    Ok(MovementResult {
        movement,
        updated_inventory: updated,
        requires_restock,
        is_out_of_stock,
    })
}
