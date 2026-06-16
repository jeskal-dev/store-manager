use crate::application::dtos::analytics::{
    DailySalesOutput, InventorySummaryOutput, PaymentMethodSalesOutput, StoreSalesOutput,
    TopProductOutput,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Repository for aggregated analytics queries.
///
/// This is an application-layer port (not a domain repository) because
/// analytics are a read-side concern that span multiple aggregates.
#[async_trait]
pub trait AnalyticsRepository: Send + Sync {
    /// Daily revenue + count in [start, end], optionally filtered by store.
    async fn get_daily_sales(
        &self,
        store_id: Option<Uuid>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<DailySalesOutput>>;

    /// Top N products by revenue in [start, end].
    async fn get_top_products(
        &self,
        store_id: Option<Uuid>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        limit: u32,
    ) -> Result<Vec<TopProductOutput>>;

    /// Aggregate inventory summary (total, low stock, out of stock, value).
    async fn get_inventory_summary(&self, store_id: Option<Uuid>)
        -> Result<InventorySummaryOutput>;

    /// Sales grouped by store in [start, end].
    async fn get_sales_by_store(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<StoreSalesOutput>>;

    /// Sales grouped by payment method in [start, end].
    async fn get_sales_by_payment_method(
        &self,
        store_id: Option<Uuid>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<PaymentMethodSalesOutput>>;
}
