use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::product::Product;
use crate::domain::repositories::product::ProductRepository;

use crate::shared::criteria::{Criteria, PaginatedResult};

use super::super::dtos::product::{CreateProductInput, UpdateProductInput};
use super::super::dtos::shared::CriteriaInput;

#[async_trait]
pub trait ForProductUseCases {
    async fn create(&self, input: CreateProductInput) -> Result<Product>;
    async fn update(&self, id: Uuid, input: UpdateProductInput) -> Result<Product>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Product>>;
}

pub struct ForProductInteractor<R: ProductRepository> {
    repo: R,
}

impl<R: ProductRepository> ForProductInteractor<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: ProductRepository + Sync> ForProductUseCases for ForProductInteractor<R> {
    async fn create(&self, input: CreateProductInput) -> Result<Product> {
        let price = input.parse_initial_price()?;
        let product = Product::new(input.product_code, input.name, price, input.active)?;
        self.repo.create(&product).await?;
        Ok(product)
    }
    async fn update(&self, id: Uuid, input: UpdateProductInput) -> Result<Product> {
        let mut product = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Product not found"))?;

        if let Some(ref code) = input.product_code {
            product.update_product_code(code.clone())?;
        }
        if let Some(ref name) = input.name {
            product.update_name(name.clone())?;
        }
        if let Some(price) = input.parse_initial_price()? {
            product.update_price(price)?;
        }
        if let Some(active) = input.active {
            if active != product.is_active() {
                product.update_active(active);
            }
        }

        self.repo.update(&product).await?;
        Ok(product)
    }
    async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await?;
        Ok(())
    }

    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Product>> {
        let criteria: Criteria<()> = input.try_into()?;
        self.repo.search(criteria.into()).await
    }
}
