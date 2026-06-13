use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::aggregates::sale::Sales;
use crate::domain::repositories::sale::SalesRepository;

use crate::shared::criteria::{Criteria, PaginatedResult};

use super::super::dtos::shared::CriteriaInput;

#[async_trait]
pub trait ForSaleUseCases {
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Sales>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Sales>;
}

pub struct ForSaleInteractor<R: SalesRepository> {
    repo: R,
}

impl<R: SalesRepository> ForSaleInteractor<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: SalesRepository + Sync> ForSaleUseCases for ForSaleInteractor<R> {
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Sales>> {
        let criteria: Criteria<()> = input.try_into()?;
        self.repo.search(criteria.into()).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Sales> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Sale not found"))
    }
}
