use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::entities::inventory::Inventory;
use crate::domain::entities::inventory_movement::InventoryMovement;
use crate::domain::repositories::inventory_movement::InventoryMovementRepository;
use crate::domain::repositories::Repository;
use crate::shared::criteria::{Criteria, PaginatedResult};

#[derive(Clone)]
pub struct SqliteInventoryMovementRepository {
    pool: SqlitePool,
}

impl SqliteInventoryMovementRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<InventoryMovement> for SqliteInventoryMovementRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<InventoryMovement>> {
        sqlx::query_as::<_, InventoryMovement>(
            "SELECT * FROM inventory_movements WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Into::into)
    }

    async fn search(
        &self,
        _criteria: Criteria<InventoryMovement>,
    ) -> Result<PaginatedResult<InventoryMovement>> {
        // TODO: Implement search with criteria when needed
        // For now, return all movements with pagination
        let items = sqlx::query_as::<_, InventoryMovement>(
            "SELECT * FROM inventory_movements ORDER BY date DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        let total = items.len() as u64;

        Ok(PaginatedResult {
            data: items,
            meta: crate::shared::criteria::PaginationMeta {
                current_page: 1,
                total_pages: 1,
                total_items: total,
            },
        })
    }

    async fn create(&self, entity: &InventoryMovement) -> Result<()> {
        sqlx::query(
            "INSERT INTO inventory_movements (id, inventory_id, movement_type, old_quantity, new_quantity, date, description, purchase_item_id, sale_item_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id)
        .bind(entity.inventory_id)
        .bind(&entity.movement_type)
        .bind(&entity.old_quantity)
        .bind(&entity.new_quantity)
        .bind(entity.date)
        .bind(&entity.description)
        .bind(entity.purchase_item_id)
        .bind(entity.sale_item_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, _entity: &InventoryMovement) -> Result<()> {
        // TODO: Inventory movements are immutable audit records — update should not be needed
        anyhow::bail!("Inventory movements cannot be updated");
    }

    async fn delete(&self, _id: Uuid) -> Result<()> {
        // TODO: Inventory movements are immutable audit records — delete should not be needed
        anyhow::bail!("Inventory movements cannot be deleted");
    }

    async fn delete_many(&self, _ids: &[Uuid]) -> Result<()> {
        anyhow::bail!("Inventory movements cannot be deleted");
    }
}

#[async_trait]
impl InventoryMovementRepository for SqliteInventoryMovementRepository {
    async fn find_by_inventory(&self, inventory_id: Uuid) -> Result<Vec<InventoryMovement>> {
        sqlx::query_as::<_, InventoryMovement>(
            "SELECT * FROM inventory_movements WHERE inventory_id = ? ORDER BY date DESC",
        )
        .bind(inventory_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    async fn find_by_movement_type(&self, movement_type: &str) -> Result<Vec<InventoryMovement>> {
        sqlx::query_as::<_, InventoryMovement>(
            "SELECT * FROM inventory_movements WHERE movement_type = ? ORDER BY date DESC",
        )
        .bind(movement_type)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    async fn find_by_month(&self, year: i32, month: u32) -> Result<Vec<InventoryMovement>> {
        sqlx::query_as::<_, InventoryMovement>(
            "SELECT * FROM inventory_movements WHERE strftime('%Y', date) = ? AND strftime('%m', date) = ? ORDER BY date DESC",
        )
        .bind(format!("{:04}", year))
        .bind(format!("{:02}", month))
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    async fn find_by_day(&self, date: DateTime<Utc>) -> Result<Vec<InventoryMovement>> {
        sqlx::query_as::<_, InventoryMovement>(
            "SELECT * FROM inventory_movements WHERE date(date) = date(?) ORDER BY date DESC",
        )
        .bind(date)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    async fn find_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<InventoryMovement>> {
        sqlx::query_as::<_, InventoryMovement>(
            "SELECT * FROM inventory_movements WHERE date >= ? AND date <= ? ORDER BY date DESC",
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    async fn save_with_inventory_update(
        &self,
        movement: &InventoryMovement,
        inventory: &Inventory,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO inventory_movements (id, inventory_id, movement_type, old_quantity, new_quantity, date, description, purchase_item_id, sale_item_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(movement.id)
        .bind(movement.inventory_id)
        .bind(&movement.movement_type)
        .bind(&movement.old_quantity)
        .bind(&movement.new_quantity)
        .bind(movement.date)
        .bind(&movement.description)
        .bind(movement.purchase_item_id)
        .bind(movement.sale_item_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "UPDATE inventory SET quantity = ?, status = ?, last_updated = ? WHERE id = ?",
        )
        .bind(&inventory.quantity)
        .bind(&inventory.status)
        .bind(inventory.last_updated)
        .bind(inventory.id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}
