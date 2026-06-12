use async_trait::async_trait;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::entities::inventory::Inventory;

use super::Repository;

#[async_trait]
pub trait InventoryRepository: Repository<Inventory> {
    async fn find_by_code(&self, code: &str) -> Result<Option<Inventory>>;
    async fn find_by_store(&self, store_id: Uuid) -> Result<Vec<Inventory>>;
    async fn find_by_product(&self, product_id: Uuid) -> Result<Vec<Inventory>>;
    async fn find_by_product_and_store(
        &self,
        product_id: Uuid,
        store_id: Uuid,
    ) -> Result<Option<Inventory>>;
    async fn find_by_status(&self, status: &str) -> Result<Vec<Inventory>>;
    async fn find_low_stock(&self, store_id: Option<Uuid>) -> Result<Vec<Inventory>>;
}
