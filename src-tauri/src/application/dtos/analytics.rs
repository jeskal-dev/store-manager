//! DTOs for dashboard analytics / chart data.
//!
//! All types are read-only view models returned by the analytics use cases.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Input DTOs
// ---------------------------------------------------------------------------

/// Input for the main dashboard query — gets everything for a period.
#[derive(Debug, Deserialize)]
pub struct DashboardInput {
    /// Optional store filter (translates to `None` in Rust).
    pub store_id: Option<String>,
    /// Number of days to look back (default 30).
    pub days: u32,
}

/// Input for a date-range-based analytics query.
#[derive(Debug, Deserialize)]
pub struct DateRangeInput {
    pub store_id: Option<String>,
    /// ISO-8601 start date.
    pub start: String,
    /// ISO-8601 end date.
    pub end: String,
}

/// Input for the top-products query.
#[derive(Debug, Deserialize)]
pub struct TopProductsInput {
    pub store_id: Option<String>,
    pub start: String,
    pub end: String,
    pub limit: u32,
}

// ---------------------------------------------------------------------------
// Output DTOs
// ---------------------------------------------------------------------------

/// One day of sales data for the revenue-over-time chart.
#[derive(Debug, Clone, Serialize)]
pub struct DailySales {
    pub date: NaiveDate,
    pub total: Decimal,
    pub count: i64,
}

/// A product ranked by revenue in the top-products chart.
#[derive(Debug, Clone, Serialize)]
pub struct TopProduct {
    pub product_id: Uuid,
    pub product_code: String,
    pub name: String,
    pub total_revenue: Decimal,
    pub units_sold: i64,
}

/// Aggregate inventory health snapshot.
#[derive(Debug, Clone, Serialize)]
pub struct InventorySummary {
    pub total_products: i64,
    pub low_stock: i64,
    pub out_of_stock: i64,
    pub total_value: Decimal,
}

/// One store's aggregated sales data.
#[derive(Debug, Clone, Serialize)]
pub struct StoreSales {
    pub store_id: Uuid,
    pub store_code: String,
    pub name: String,
    pub total_revenue: Decimal,
    pub sales_count: i64,
}

/// One payment method's aggregated data.
#[derive(Debug, Clone, Serialize)]
pub struct PaymentMethodSales {
    pub method: String,
    pub total_revenue: Decimal,
    pub count: i64,
}

/// Combined payload returned by `get_dashboard_data`.
#[derive(Debug, Clone, Serialize)]
pub struct DashboardData {
    pub sales_over_time: Vec<DailySales>,
    pub top_products: Vec<TopProduct>,
    pub inventory_summary: InventorySummary,
    pub sales_by_store: Vec<StoreSales>,
    pub sales_by_payment: Vec<PaymentMethodSales>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}
