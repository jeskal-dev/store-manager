use serde::Deserialize;
use validator::Validate;

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
