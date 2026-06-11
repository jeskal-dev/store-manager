use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::supply_agreement::SupplyAgreement;
use crate::domain::repositories::supply_agreement::SupplyAgreementRepository;

use super::super::dtos::supply_agreement::{
    CreateSupplyAgreementInput, UpdateSupplyAgreementInput,
};

#[async_trait]
pub trait ForSupplyAgreementUseCases {
    async fn create(&self, input: CreateSupplyAgreementInput) -> Result<SupplyAgreement>;
    async fn update(&self, id: Uuid, input: UpdateSupplyAgreementInput) -> Result<SupplyAgreement>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct ForSupplyAgreementInteractor<R: SupplyAgreementRepository> {
    repo: R,
}

impl<R: SupplyAgreementRepository> ForSupplyAgreementInteractor<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: SupplyAgreementRepository + Sync> ForSupplyAgreementUseCases
    for ForSupplyAgreementInteractor<R>
{
    async fn create(&self, input: CreateSupplyAgreementInput) -> Result<SupplyAgreement> {
        let product_id = Uuid::parse_str(&input.product_id)?;
        let supplier_id = Uuid::parse_str(&input.supplier_id)?;
        let cost = input.parse_cost()?;

        let agreement = SupplyAgreement::new(product_id, supplier_id, cost, input.active)?;

        self.repo.create(&agreement).await?;
        Ok(agreement)
    }
    async fn update(&self, id: Uuid, input: UpdateSupplyAgreementInput) -> Result<SupplyAgreement> {
        let mut agreement = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Supply agreement not found"))?;

        if let Some(ref product_id) = input.product_id {
            agreement.product_id = Uuid::parse_str(product_id)?;
        }
        if let Some(ref supplier_id) = input.supplier_id {
            agreement.supplier_id = Uuid::parse_str(supplier_id)?;
        }
        if let Some(cost) = input.parse_cost()? {
            agreement.update_cost(cost)?;
        }
        if let Some(active) = input.active {
            if active {
                agreement.activate();
            } else {
                agreement.deactivate();
            }
        }

        self.repo.update(&agreement).await?;
        Ok(agreement)
    }
    async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await?;
        Ok(())
    }
}
