use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSupplyAgreementInput {
    #[validate]
    pub product_id: String,

    #[validate]
    pub supplier_id: String,

    pub cost: Option<f64>,

    pub active: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSupplyAgreementInput {
    #[validate]
    #[serde(default)]
    pub product_id: Option<String>,
    #[validate]
    #[serde(default)]
    pub supplier_id: Option<String>,
    #[serde(default)]
    pub cost: Option<Option<f64>>,
    #[serde(default)]
    pub active: Option<bool>,
}