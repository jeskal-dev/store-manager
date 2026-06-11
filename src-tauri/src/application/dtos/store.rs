use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStoreInput {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Store code must be between 1 and 20 characters"
    ))]
    pub store_code: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(length(min = 1, message = "Address cannot be empty"))]
    pub address: String,

    #[validate(length(min = 7, max = 20, message = "Phone number length is invalid"))]
    #[validate(regex(
        path = "crate::shared::validators::PHONE_REGEX",
        message = "Phone number format is invalid"
    ))]
    pub phone: Option<String>,

    pub active: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStoreInput {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Store code must be between 1 and 20 characters"
    ))]
    #[serde(default)]
    // Si la clave no viene en el JSON, será None. Si viene null, será Some(None)
    pub store_code: Option<String>,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    #[serde(default)]
    pub name: Option<String>,
    #[validate(length(min = 1, message = "Address cannot be empty"))]
    #[serde(default)]
    pub address: Option<String>,
    #[validate(length(min = 7, max = 20, message = "Phone number length is invalid"))]
    #[validate(regex(
        path = "crate::shared::validators::PHONE_REGEX",
        message = "Phone number format is invalid"
    ))]
    #[serde(default)]
    pub phone: Option<Option<String>>,
    #[serde(default)]
    pub active: Option<bool>,
}
