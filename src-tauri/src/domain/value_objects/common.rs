use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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
