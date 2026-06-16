use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::{
    aggregates::{purchase::Purchase, sale::Sale},
    value_objects::sale::PaymentMethod,
};

// Row types for all repository entities
// These are internal types used for database mapping

#[derive(FromRow)]
pub struct SalesRow {
    pub id: Uuid,
    pub store_id: Uuid,
    pub sale_code: String,
    pub total: String,
    pub payment_method: String,
    pub sale_date: DateTime<Utc>,
}

impl From<SalesRow> for Sale {
    fn from(val: SalesRow) -> Self {
        Sale::restore(
            val.id,
            val.store_id,
            val.sale_code,
            Decimal::from_str(&val.total).unwrap_or(Decimal::ZERO),
            PaymentMethod::from_str(&val.payment_method).unwrap(),
            val.sale_date,
            vec![],
        )
        .unwrap()
    }
}

#[derive(FromRow)]
pub struct PurchaseRow {
    pub id: Uuid,
    pub supplier_id: Option<Uuid>,
    pub store_id: Uuid,
    pub total_cost: String,
    pub purchase_date: DateTime<Utc>,
    pub description: Option<String>,
    pub purchase_code: String,
}

impl From<PurchaseRow> for Purchase {
    fn from(val: PurchaseRow) -> Self {
        Purchase::restore(
            val.id,
            val.supplier_id,
            val.store_id,
            Decimal::from_str(&val.total_cost).unwrap_or(Decimal::ZERO),
            val.purchase_date,
            val.description,
            val.purchase_code,
            vec![],
        )
        .unwrap()
    }
}

#[derive(FromRow)]
pub struct DailySalesRow {
    pub date: NaiveDate,
    pub total: String,
    pub count: i64,
}

#[derive(FromRow)]
pub struct TopProductRow {
    pub product_id: String,
    pub product_code: String,
    pub name: String,
    pub total_revenue: String,
    pub units_sold: i64,
}

#[derive(FromRow)]
pub struct InventorySummaryRow {
    pub total_products: i64,
    pub low_stock: i64,
    pub out_of_stock: i64,
    pub total_value: String,
}

#[derive(FromRow)]
pub struct StoreSalesRow {
    pub store_id: String,
    pub store_code: String,
    pub name: String,
    pub total_revenue: String,
    pub sales_count: i64,
}

#[derive(FromRow)]
pub struct PaymentMethodSalesRow {
    pub method: String,
    pub total_revenue: String,
    pub count: i64,
}
