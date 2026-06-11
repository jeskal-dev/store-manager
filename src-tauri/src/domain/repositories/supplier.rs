use async_trait::async_trait;

use anyhow::Result;

use crate::domain::entities::supplier::Supplier;

use super::Repository;

#[async_trait]
pub trait SupplierRepository: Repository<Supplier> {
    async fn find_by_code(&self, code: &str) -> Result<Option<Supplier>>;
    async fn exists_by_code(&self, code: &str) -> Result<bool>;
}
