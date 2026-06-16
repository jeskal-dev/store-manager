use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::aggregates::purchase::Purchase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseCompleted {
    pub purchase_id: Uuid,
    pub store_id: Uuid,
    pub supplier_id: Option<Uuid>,
    pub total_cost: Decimal,
    pub purchase_code: String,
    pub occurred_at: DateTime<Utc>,
}

impl PurchaseCompleted {
    pub fn from_purchase(purchase: &Purchase) -> Self {
        Self {
            purchase_id: purchase.id,
            store_id: purchase.store_id,
            supplier_id: purchase.supplier_id,
            total_cost: purchase.total_cost.amount(),
            purchase_code: purchase.purchase_code.value().to_string(),
            occurred_at: purchase.purchase_date,
        }
    }
}
