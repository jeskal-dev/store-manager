use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::aggregates::sale::Sale;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleCompleted {
    pub sale_id: Uuid,
    pub store_id: Uuid,
    pub total: Decimal,
    pub sale_code: String,
    pub payment_method: String,
    pub occurred_at: DateTime<Utc>,
}

impl SaleCompleted {
    pub fn from_sale(sale: &Sale) -> Self {
        Self {
            sale_id: sale.id,
            store_id: sale.store_id,
            total: sale.total.amount(),
            sale_code: sale.sale_code.value().to_string(),
            payment_method: sale.payment_method.to_string(),
            occurred_at: sale.sale_date,
        }
    }
}
