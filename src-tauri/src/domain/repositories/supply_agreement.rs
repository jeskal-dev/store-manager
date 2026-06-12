use async_trait::async_trait;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::entities::supply_agreement::SupplyAgreement;

use super::Repository;

#[async_trait]
pub trait SupplyAgreementRepository: Repository<SupplyAgreement> {
    async fn find_by_product(&self, product_id: Uuid) -> Result<Vec<SupplyAgreement>>;
    async fn find_by_supplier(&self, supplier_id: Uuid) -> Result<Vec<SupplyAgreement>>;
    async fn find_by_product_and_supplier(
        &self,
        product_id: Uuid,
        supplier_id: Uuid,
    ) -> Result<Option<SupplyAgreement>>;
}
