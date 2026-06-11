use async_trait::async_trait;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::aggregates::sale::Sales;

use super::Repository;

#[async_trait]
pub trait SalesRepository: Repository<Sales> {
    async fn find_by_code(&self, code: &str) -> Result<Option<Sales>>;
    async fn find_by_store(&self, store_id: Uuid) -> Result<Vec<Sales>>;
}
