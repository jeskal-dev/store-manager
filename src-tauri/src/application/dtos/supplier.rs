use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::supplier::Supplier;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSupplierInput {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 20,
        message = "Supplier code must be between 1 and 20 characters"
    ))]
    pub supplier_code: String,

    #[validate(length(min = 1, message = "Contact name cannot be empty"))]
    pub contact_name: Option<String>,

    #[validate(length(min = 7, max = 20, message = "Phone number length is invalid"))]
    #[validate(regex(
        path = "crate::shared::validators::PHONE_REGEX",
        message = "Phone number format is invalid"
    ))]
    pub phone: Option<String>,

    #[validate(length(min = 1, message = "Address cannot be empty"))]
    pub address: Option<String>,

    pub active: bool,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSupplierInput {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    #[serde(default)]
    pub name: Option<String>,
    #[validate(length(
        min = 1,
        max = 20,
        message = "Supplier code must be between 1 and 20 characters"
    ))]
    #[serde(default)]
    pub supplier_code: Option<String>,
    #[validate(length(min = 1, message = "Contact name cannot be empty"))]
    #[serde(default)]
    pub contact_name: Option<Option<String>>,
    #[validate(length(min = 7, max = 20, message = "Phone number length is invalid"))]
    #[validate(regex(
        path = "crate::shared::validators::PHONE_REGEX",
        message = "Phone number format is invalid"
    ))]
    #[serde(default)]
    pub phone: Option<Option<String>>,
    #[validate(length(min = 1, message = "Address cannot be empty"))]
    #[serde(default)]
    pub address: Option<Option<String>>,
    #[serde(default)]
    pub active: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplierOutput {
    pub id: String,
    pub name: String,
    pub supplier_code: String,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub active: bool,
}

impl From<&Supplier> for SupplierOutput {
    fn from(supplier: &Supplier) -> Self {
        Self {
            id: supplier.id.to_string(),
            name: supplier.name.value().to_string(),
            supplier_code: supplier.supplier_code.value().to_string(),
            contact_name: supplier.contact_name.as_ref().map(|cn| cn.value().to_string()),
            phone: supplier.phone.as_ref().map(|p| p.value().to_string()),
            address: supplier.address.as_ref().map(|a| a.value().to_string()),
            active: supplier.active,
        }
    }
}
