use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{
    decode::Decode,
    encode::IsNull,
    error::BoxDynError,
    sqlite::{SqliteArgumentsBuffer, SqliteTypeInfo, SqliteValueRef},
    Encode, Sqlite, Type,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryStatus {
    #[serde(rename = "IN_STOCK")]
    InStock,
    #[serde(rename = "LOW_STOCK")]
    LowStock,
    #[serde(rename = "OUT_OF_STOCK")]
    OutOfStock,
}

impl InventoryStatus {
    pub fn new(value: String) -> Result<Self> {
        match value.to_uppercase().as_str() {
            "IN_STOCK" => Ok(Self::InStock),
            "LOW_STOCK" => Ok(Self::LowStock),
            "OUT_OF_STOCK" => Ok(Self::OutOfStock),
            _ => Err(anyhow::anyhow!("Invalid inventory status")),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InStock => "IN_STOCK",
            Self::LowStock => "LOW_STOCK",
            Self::OutOfStock => "OUT_OF_STOCK",
        }
    }
}

impl Type<Sqlite> for InventoryStatus {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for InventoryStatus {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        let s = self.as_str().to_string();
        <String as Encode<Sqlite>>::encode_by_ref(&s, args)
    }
}

impl Decode<'_, Sqlite> for InventoryStatus {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self::new(s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MovementType {
    #[serde(rename = "restock")]
    Restock,
    #[serde(rename = "shrinkage")]
    Shrinkage,
    #[serde(rename = "sale")]
    Sale,
    #[serde(rename = "purchase")]
    Purchase,
}

impl MovementType {
    pub fn new(value: String) -> Result<Self> {
        match value.to_lowercase().as_str() {
            "restock" => Ok(Self::Restock),
            "shrinkage" => Ok(Self::Shrinkage),
            "sale" => Ok(Self::Sale),
            "purchase" => Ok(Self::Purchase),
            _ => Err(anyhow::anyhow!("Invalid movement type")),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Restock => "restock",
            Self::Shrinkage => "shrinkage",
            Self::Sale => "sale",
            Self::Purchase => "purchase",
        }
    }
}

impl Type<Sqlite> for MovementType {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for MovementType {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        <String as Encode<Sqlite>>::encode_by_ref(&self.as_str().to_string(), args)
    }
}

impl Decode<'_, Sqlite> for MovementType {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self::new(s)?)
    }
}
