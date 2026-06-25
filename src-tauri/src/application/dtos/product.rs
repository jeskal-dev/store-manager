use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::product::Product;


#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOutput {
    pub id: String,
    pub product_code: String,
    pub name: String,
    pub initial_price: String,
    pub active: bool,
}

impl From<&Product> for ProductOutput {
    fn from(product: &Product) -> Self {
        Self {
            id: product.id.to_string(),
            product_code: product.product_code.value().to_string(),
            name: product.name.value().to_string(),
            initial_price: product.initial_price.formatted(),
            active: product.active,
        }
    }
}
