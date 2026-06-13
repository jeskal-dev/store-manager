use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::aggregates::purchase::Purchase;
use crate::domain::repositories::purchase::PurchaseRepository;

use crate::shared::criteria::{Criteria, PaginatedResult};

use super::super::dtos::shared::CriteriaInput;

#[async_trait]
pub trait ForPurchaseUseCases {
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Purchase>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Purchase>;
}

pub struct ForPurchaseInteractor<R: PurchaseRepository> {
    repo: R,
}

impl<R: PurchaseRepository> ForPurchaseInteractor<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: PurchaseRepository + Sync> ForPurchaseUseCases for ForPurchaseInteractor<R> {
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<Purchase>> {
        let criteria: Criteria<()> = input.try_into()?;
        self.repo.search(criteria.into()).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Purchase> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Purchase not found"))
    }
}
