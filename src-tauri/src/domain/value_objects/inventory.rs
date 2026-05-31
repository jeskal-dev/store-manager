use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryStatus {
    value: String,
}

impl InventoryStatus {
    pub fn new(value: String) -> Result<Self> {
        let status = Self {
            value: value.to_uppercase(),
        };
        status.validate()?;
        Ok(status)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn validate(&self) -> Result<()> {
        match self.value.as_str() {
            "ACTIVE" | "INACTIVE" | "PENDING" | "DISCONTINUED" => Ok(()),
            _ => Err(anyhow::anyhow!("Invalid inventory status")),
        }
    }
}
