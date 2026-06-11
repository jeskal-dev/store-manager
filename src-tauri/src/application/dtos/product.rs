use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
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

    #[validate]
    pub initial_price: f64,

    pub active: bool,
}

#[derive(Debug, Deserialize, Validate)]
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
    #[validate]
    #[serde(default)]
    pub initial_price: Option<f64>,
    #[serde(default)]
    pub active: Option<bool>,
}