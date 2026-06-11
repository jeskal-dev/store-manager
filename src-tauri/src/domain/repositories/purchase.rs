use async_trait::async_trait;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::aggregates::purchase::Purchase;

use super::Repository;

#[async_trait]
pub trait PurchaseRepository: Repository<Purchase> {
    async fn find_by_code(&self, code: &str) -> Result<Option<Purchase>>;
    async fn find_by_store(&self, store_id: Uuid) -> Result<Vec<Purchase>>;
    async fn find_by_supplier(&self, supplier_id: Uuid) -> Result<Vec<Purchase>>;
}
