use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::supply_agreement::SupplyAgreement;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSupplyAgreementInput {
    pub product_id: String,
    pub supplier_id: String,
    pub cost: Option<String>,
    pub active: bool,
}

impl CreateSupplyAgreementInput {
    pub fn parse_cost(&self) -> anyhow::Result<Option<Decimal>> {
        match &self.cost {
            Some(s) => Ok(Some(s.parse::<Decimal>()?)),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSupplyAgreementInput {
    #[serde(default)]
    pub product_id: Option<String>,
    #[serde(default)]
    pub supplier_id: Option<String>,
    #[serde(default)]
    pub cost: Option<Option<String>>,
    #[serde(default)]
    pub active: Option<bool>,
}

impl UpdateSupplyAgreementInput {
    pub fn parse_cost(&self) -> anyhow::Result<Option<Option<Decimal>>> {
        match &self.cost {
            Some(Some(s)) => Ok(Some(Some(s.parse::<Decimal>()?))),
            Some(None) => Ok(Some(None)),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyAgreementOutput {
    pub id: String,
    pub product_id: String,
    pub supplier_id: String,
    pub cost: Option<String>,
    pub active: bool,
}

impl From<&SupplyAgreement> for SupplyAgreementOutput {
    fn from(agreement: &SupplyAgreement) -> Self {
        Self {
            id: agreement.id.to_string(),
            product_id: agreement.product_id.to_string(),
            supplier_id: agreement.supplier_id.to_string(),
            cost: agreement.cost.as_ref().map(|c| c.formatted()),
            active: agreement.active,
        }
    }
}
