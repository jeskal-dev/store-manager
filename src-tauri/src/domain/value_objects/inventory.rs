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

impl Type<Sqlite> for InventoryStatus {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for InventoryStatus {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        <String as Encode<Sqlite>>::encode_by_ref(&self.value, args)
    }
}

impl Decode<'_, Sqlite> for InventoryStatus {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self::new(s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementType {
    value: String,
}

impl MovementType {
    pub fn new(value: String) -> Result<Self> {
        let mt = Self {
            value: value.to_lowercase(),
        };
        mt.validate()?;
        Ok(mt)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn validate(&self) -> Result<()> {
        match self.value.as_str() {
            "restock" | "shrinkage" | "sale" | "purchase" => Ok(()),
            _ => Err(anyhow::anyhow!("Invalid movement type")),
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
        <String as Encode<Sqlite>>::encode_by_ref(&self.value, args)
    }
}

impl Decode<'_, Sqlite> for MovementType {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self::new(s)?)
    }
}
