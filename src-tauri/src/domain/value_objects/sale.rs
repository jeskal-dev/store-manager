use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{
    decode::Decode,
    encode::IsNull,
    error::BoxDynError,
    sqlite::{SqliteArgumentsBuffer, SqliteTypeInfo, SqliteValueRef},
    Encode, Sqlite, Type,
};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum PaymentMethod {
    #[strum(serialize = "cash")]
    Cash,
    #[strum(serialize = "credit_card")]
    CreditCard,
    #[strum(serialize = "bank_transfer")]
    BankTransfer,
    #[strum(serialize = "other")]
    Other,
}

impl FromStr for PaymentMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cash" => Ok(Self::Cash),
            "credit_card" => Ok(Self::CreditCard),
            "bank_transfer" => Ok(Self::BankTransfer),
            "other" => Ok(Self::Other),
            _ => Err(anyhow::anyhow!("Invalid payment method: {}", s)),
        }
    }
}

impl TryFrom<&str> for PaymentMethod {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl Type<Sqlite> for PaymentMethod {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for PaymentMethod {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        let s = self.to_string();
        <String as Encode<Sqlite>>::encode_by_ref(&s, args)
    }
}

impl Decode<'_, Sqlite> for PaymentMethod {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        match s.to_lowercase().as_str() {
            "cash" => Ok(Self::Cash),
            "credit_card" => Ok(Self::CreditCard),
            "bank_transfer" => Ok(Self::BankTransfer),
            _ => Ok(Self::Other),
        }
    }
}
