use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{
    decode::Decode,
    encode::IsNull,
    error::BoxDynError,
    sqlite::{SqliteArgumentsBuffer, SqliteTypeInfo, SqliteValueRef},
    Encode, Sqlite, Type,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextValue {
    value: String,
    max_length: usize,
}

impl TextValue {
    pub fn new(value: String, max_length: usize) -> Result<Self> {
        let text = Self { value, max_length };
        text.validate()?;
        Ok(text)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn validate(&self) -> Result<()> {
        if self.value.trim().is_empty() {
            return Err(anyhow::anyhow!("Text cannot be empty"));
        }
        if self.value.len() > self.max_length {
            return Err(anyhow::anyhow!(format!(
                "Text exceeds maximum length of {}",
                self.max_length
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Code {
    value: TextValue,
}

impl Code {
    pub fn new(value: String) -> Result<Self> {
        let text_value = TextValue::new(value, 50)?;
        Ok(Self { value: text_value })
    }

    pub fn value(&self) -> &str {
        self.value.value()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    value: TextValue,
}

impl Name {
    pub fn new(value: String) -> Result<Self> {
        let text_value = TextValue::new(value, 255)?;
        Ok(Self { value: text_value })
    }

    pub fn value(&self) -> &str {
        self.value.value()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Money {
    amount: Decimal,
}

impl Money {
    pub fn new(amount: Decimal) -> Result<Self> {
        let money = Self { amount };
        money.validate()?;
        Ok(money)
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn formatted(&self) -> String {
        format!("{:.2}", self.amount)
    }

    fn validate(&self) -> Result<()> {
        if self.amount < Decimal::ZERO {
            return Err(anyhow::anyhow!("Amount cannot be negative"));
        }
        if self.amount.scale() > 4 {
            return Err(anyhow::anyhow!(
                "Amount cannot have more than 4 decimal places"
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
    value: String,
}

impl PhoneNumber {
    pub fn new(value: String) -> Result<Self> {
        let phone = Self {
            value: value.trim().to_string(),
        };
        phone.validate()?;
        Ok(phone)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn validate(&self) -> Result<()> {
        if self.value.is_empty() {
            return Err(anyhow::anyhow!("Phone number cannot be empty"));
        }
        if self.value.len() > 15 {
            return Err(anyhow::anyhow!("Phone number exceeds maximum length of 15"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    value: i32,
}

impl Quantity {
    pub fn new(value: i32) -> Result<Self> {
        if value < 0 {
            return Err(anyhow::anyhow!("Quantity cannot be negative"));
        }
        Ok(Self { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// ── sqlx impls for value objects ──

impl Type<Sqlite> for TextValue {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for TextValue {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        <String as Encode<Sqlite>>::encode_by_ref(&self.value, args)
    }
}

impl Decode<'_, Sqlite> for TextValue {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        let max_len = s.len();
        Ok(Self { value: s, max_length: max_len })
    }
}

impl Type<Sqlite> for Code {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for Code {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        <String as Encode<Sqlite>>::encode_by_ref(&self.value.value().to_string(), args)
    }
}

impl Decode<'_, Sqlite> for Code {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self {
            value: TextValue::new(s, 50)?,
        })
    }
}

impl Type<Sqlite> for Name {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for Name {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        <String as Encode<Sqlite>>::encode_by_ref(&self.value.value().to_string(), args)
    }
}

impl Decode<'_, Sqlite> for Name {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self {
            value: TextValue::new(s, 255)?,
        })
    }
}

impl Type<Sqlite> for PhoneNumber {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for PhoneNumber {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        <String as Encode<Sqlite>>::encode_by_ref(&self.value, args)
    }
}

impl Decode<'_, Sqlite> for PhoneNumber {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self::new(s)?)
    }
}

impl Type<Sqlite> for Money {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for Money {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        let s = self.amount.to_string();
        <String as Encode<Sqlite>>::encode_by_ref(&s, args)
    }
}

impl Decode<'_, Sqlite> for Money {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        let d: Decimal = s.parse()?;
        Ok(Self::new(d)?)
    }
}

impl Type<Sqlite> for Quantity {
    fn type_info() -> SqliteTypeInfo {
        <i32 as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for Quantity {
    fn encode_by_ref(&self, args: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        <i32 as Encode<Sqlite>>::encode_by_ref(&self.value, args)
    }
}

impl Decode<'_, Sqlite> for Quantity {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let i = <i32 as Decode<Sqlite>>::decode(value)?;
        Ok(Self { value: i })
    }
}
