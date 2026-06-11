use async_trait::async_trait;

use anyhow::Result;

use crate::domain::entities::store::Store;

use super::Repository;

#[async_trait]
pub trait StoreRepository: Repository<Store> {
    async fn find_by_code(&self, code: &str) -> Result<Option<Store>>;
    async fn exists_by_code(&self, code: &str) -> Result<bool>;
}
