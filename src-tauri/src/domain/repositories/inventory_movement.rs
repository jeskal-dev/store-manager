use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use anyhow::Result;

use crate::domain::entities::inventory::Inventory;
use crate::domain::entities::inventory_movement::InventoryMovement;

use super::Repository;

#[async_trait]
pub trait InventoryMovementRepository: Repository<InventoryMovement> {
    async fn find_by_inventory(&self, inventory_id: Uuid) -> Result<Vec<InventoryMovement>>;
    async fn find_by_movement_type(&self, movement_type: &str) -> Result<Vec<InventoryMovement>>;
    async fn find_by_month(&self, year: i32, month: u32) -> Result<Vec<InventoryMovement>>;
    async fn find_by_day(&self, date: DateTime<Utc>) -> Result<Vec<InventoryMovement>>;
    async fn find_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<InventoryMovement>>;

    /// Persist a movement and update inventory atomically (transactional).
    async fn save_with_inventory_update(
        &self,
        movement: &InventoryMovement,
        inventory: &Inventory,
    ) -> Result<()>;
}
