use crate::domain::{
    entities::sale_item::SaleItem,
    value_objects::{
        common::{Code, Money},
        sale::PaymentMethod,
    },
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sale {
    pub id: Uuid,
    pub store_id: Uuid,
    pub sale_code: Code,
    pub total: Money,
    pub payment_method: PaymentMethod,
    pub sale_date: DateTime<Utc>,
    pub items: Vec<SaleItem>,
}

impl Sale {
    pub fn new(
        store_id: Uuid,
        sale_code: String,
        payment_method: PaymentMethod,
        items: Vec<SaleItem>,
    ) -> Result<Self> {
        let total = Self::calculate_total(&items)?;

        let sales = Self {
            id: Uuid::new_v4(),
            store_id,
            sale_code: Code::new(sale_code)?,
            total: Money::new(total)?,
            payment_method,
            sale_date: Utc::now(),
            items,
        };

        Ok(sales)
    }

    pub fn restore(
        id: Uuid,
        store_id: Uuid,
        sale_code: String,
        total: Decimal,
        payment_method: PaymentMethod,
        sale_date: DateTime<Utc>,
        items: Vec<SaleItem>,
    ) -> Result<Self> {
        let sales = Self {
            id,
            store_id,
            sale_code: Code::new(sale_code)?,
            total: Money::new(total)?,
            payment_method,
            sale_date,
            items,
        };

        Ok(sales)
    }

    pub fn add_item(&mut self, item: SaleItem) -> Result<()> {
        self.items.push(item);
        let new_total = Self::calculate_total(&self.items)?;
        self.total = Money::new(new_total)?;
        Ok(())
    }

    pub fn remove_item(&mut self, item_index: usize) -> Result<()> {
        if item_index >= self.items.len() {
            return Err(anyhow::anyhow!("Item index out of bounds"));
        }
        self.items.remove(item_index);
        let new_total = Self::calculate_total(&self.items)?;
        self.total = Money::new(new_total)?;
        Ok(())
    }

    pub fn update_item(&mut self, item_index: usize, item: SaleItem) -> Result<()> {
        if item_index >= self.items.len() {
            return Err(anyhow::anyhow!("Item index out of bounds"));
        }
        self.items[item_index] = item;
        let new_total = Self::calculate_total(&self.items)?;
        self.total = Money::new(new_total)?;
        Ok(())
    }

    pub fn calculate_total(items: &[SaleItem]) -> Result<Decimal> {
        let mut total = Decimal::ZERO;
        for item in items {
            total += item.calculate_subtotal();
        }
        Ok(total)
    }

    pub fn get_items_count(&self) -> usize {
        self.items.len()
    }

    pub fn update_payment_method(&mut self, new_method: PaymentMethod) {
        self.payment_method = new_method;
    }

    pub fn get_items(&self) -> &[SaleItem] {
        &self.items
    }

    pub fn get_mut_items(&mut self) -> &mut Vec<SaleItem> {
        &mut self.items
    }
}
