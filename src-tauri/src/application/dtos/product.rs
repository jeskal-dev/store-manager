use rust_decimal::Decimal;
use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, TS)]
#[ts(export)]
pub struct CreateProductInput {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Product code must be between 1 and 20 characters"
    ))]
    pub product_code: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    pub initial_price: String,

    pub active: bool,
}

impl CreateProductInput {
    pub fn parse_initial_price(&self) -> anyhow::Result<Decimal> {
        Ok(self.initial_price.parse::<Decimal>()?)
    }
}

#[derive(Debug, Deserialize, Validate, TS)]
#[ts(export)]
pub struct UpdateProductInput {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Product code must be between 1 and 20 characters"
    ))]
    #[serde(default)]
    pub product_code: Option<String>,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub initial_price: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
}

impl UpdateProductInput {
    pub fn parse_initial_price(&self) -> anyhow::Result<Option<Decimal>> {
        match &self.initial_price {
            Some(s) => Ok(Some(s.parse::<Decimal>()?)),
            None => Ok(None),
        }
    }
}
