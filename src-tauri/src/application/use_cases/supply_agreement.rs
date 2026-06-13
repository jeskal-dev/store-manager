use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::supply_agreement::SupplyAgreement;
use crate::domain::repositories::supply_agreement::SupplyAgreementRepository;

use crate::shared::criteria::{Criteria, PaginatedResult};

use super::super::dtos::shared::CriteriaInput;
use super::super::dtos::supply_agreement::{
    CreateSupplyAgreementInput, UpdateSupplyAgreementInput,
};

#[async_trait]
pub trait ForSupplyAgreementUseCases {
    async fn create(&self, input: CreateSupplyAgreementInput) -> Result<SupplyAgreement>;
    async fn update(&self, id: Uuid, input: UpdateSupplyAgreementInput) -> Result<SupplyAgreement>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<SupplyAgreement>>;
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
        let exists_relation = self
            .repo
            .find_by_product_and_supplier(product_id, supplier_id)
            .await?
            .is_some();

        if exists_relation {
            anyhow::bail!("A supply agreement already exists for this product and supplier");
        }

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

        let new_product_id = match &input.product_id {
            Some(pid) => Uuid::parse_str(pid)?,
            None => agreement.product_id,
        };
        let new_supplier_id = match &input.supplier_id {
            Some(sid) => Uuid::parse_str(sid)?,
            None => agreement.supplier_id,
        };

        if (input.product_id.is_some() || input.supplier_id.is_some())
            && (new_product_id != agreement.product_id || new_supplier_id != agreement.supplier_id)
        {
            if let Some(existing) = self
                .repo
                .find_by_product_and_supplier(new_product_id, new_supplier_id)
                .await?
            {
                if existing.id != id {
                    anyhow::bail!(
                        "A supply agreement already exists for this product and supplier"
                    );
                }
            }
        }

        agreement.product_id = new_product_id;
        agreement.supplier_id = new_supplier_id;
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

    async fn search(&self, input: CriteriaInput) -> Result<PaginatedResult<SupplyAgreement>> {
        let criteria: Criteria<()> = input.try_into()?;
        self.repo.search(criteria.into()).await
    }
}
