use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    entities::purchase_item::PurchaseItem,
    value_objects::common::{Code, Money, TextValue},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Purchase {
    pub id: Uuid,
    pub supplier_id: Option<Uuid>,
    pub store_id: Uuid,
    pub total_cost: Money,
    pub purchase_date: DateTime<Utc>,
    pub description: Option<TextValue>,
    pub purchase_code: Code,
    pub items: Vec<PurchaseItem>,
}

impl Purchase {
    pub fn new(
        supplier_id: Option<Uuid>,
        store_id: Uuid,
        description: Option<String>,
        purchase_code: String,
        items: Vec<PurchaseItem>,
    ) -> Result<Self> {
        let total_cost = Self::calculate_total_cost(&items)?;

        let purchase = Self {
            id: Uuid::new_v4(),
            supplier_id,
            store_id,
            total_cost: Money::new(total_cost)?,
            purchase_date: Utc::now(),
            description: match description {
                Some(desc) => Some(TextValue::new(desc, 500)?),
                None => None,
            },
            purchase_code: Code::new(purchase_code)?,
            items,
        };

        Ok(purchase)
    }

    pub fn restore(
        id: Uuid,
        supplier_id: Option<Uuid>,
        store_id: Uuid,
        total_cost: Decimal,
        purchase_date: DateTime<Utc>,
        description: Option<String>,
        purchase_code: String,
        items: Vec<PurchaseItem>,
    ) -> Result<Self> {
        let purchase = Self {
            id,
            supplier_id,
            store_id,
            total_cost: Money::new(total_cost)?,
            purchase_date,
            description: match description {
                Some(desc) => Some(TextValue::new(desc, 500)?),
                None => None,
            },
            purchase_code: Code::new(purchase_code)?,
            items,
        };

        Ok(purchase)
    }

    pub fn add_item(&mut self, item: PurchaseItem) -> Result<()> {
        self.items.push(item);
        let new_total = Self::calculate_total_cost(&self.items)?;
        self.total_cost = Money::new(new_total)?;
        Ok(())
    }

    pub fn remove_item(&mut self, item_index: usize) -> Result<()> {
        if item_index >= self.items.len() {
            return Err(anyhow::anyhow!("Item index out of bounds"));
        }
        self.items.remove(item_index);
        let new_total = Self::calculate_total_cost(&self.items)?;
        self.total_cost = Money::new(new_total)?;
        Ok(())
    }

    pub fn update_item(&mut self, item_index: usize, item: PurchaseItem) -> Result<()> {
        if item_index >= self.items.len() {
            return Err(anyhow::anyhow!("Item index out of bounds"));
        }
        self.items[item_index] = item;
        let new_total = Self::calculate_total_cost(&self.items)?;
        self.total_cost = Money::new(new_total)?;
        Ok(())
    }

    pub fn calculate_total_cost(items: &[PurchaseItem]) -> Result<Decimal> {
        let mut total = Decimal::ZERO;
        for item in items {
            total += item.calculate_subtotal();
        }
        Ok(total)
    }

    pub fn get_items_count(&self) -> usize {
        self.items.len()
    }

    pub fn update_description(&mut self, description: Option<String>) -> Result<()> {
        self.description = match description {
            Some(desc) => Some(TextValue::new(desc, 500)?),
            None => None,
        };
        Ok(())
    }

    pub fn get_items(&self) -> &[PurchaseItem] {
        &self.items
    }

    pub fn get_mut_items(&mut self) -> &mut Vec<PurchaseItem> {
        &mut self.items
    }
}
