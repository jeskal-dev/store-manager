use async_trait::async_trait;

use anyhow::Result;

use crate::domain::entities::product::Product;

use super::Repository;

#[async_trait]
pub trait ProductRepository: Repository<Product> {
    async fn find_by_code(&self, code: &str) -> Result<Option<Product>>;
    async fn exists_by_code(&self, code: &str) -> Result<bool>;
}
