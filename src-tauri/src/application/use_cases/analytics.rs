//! Analytics use cases — read-only aggregated queries for dashboard charts.
//!
//! The interactor depends on an [`AnalyticsRepository`] trait that is defined
//! here (application port) and implemented in the infrastructure layer.

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Days, Utc};
use uuid::Uuid;

use super::super::dtos::analytics::{
    DashboardData, DashboardInput, DateRangeInput, DailySales, InventorySummary,
    PaymentMethodSales, StoreSales, TopProduct, TopProductsInput,
};

// ---------------------------------------------------------------------------
// Application port — implemented by infrastructure
// ---------------------------------------------------------------------------

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
    ) -> Result<Vec<DailySales>>;

    /// Top N products by revenue in [start, end].
    async fn get_top_products(
        &self,
        store_id: Option<Uuid>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        limit: u32,
    ) -> Result<Vec<TopProduct>>;

    /// Aggregate inventory summary (total, low stock, out of stock, value).
    async fn get_inventory_summary(&self, store_id: Option<Uuid>) -> Result<InventorySummary>;

    /// Sales grouped by store in [start, end].
    async fn get_sales_by_store(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<StoreSales>>;

    /// Sales grouped by payment method in [start, end].
    async fn get_sales_by_payment_method(
        &self,
        store_id: Option<Uuid>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<PaymentMethodSales>>;
}

// ---------------------------------------------------------------------------
// Use case trait
// ---------------------------------------------------------------------------

#[async_trait]
pub trait ForAnalyticsUseCases {
    /// Convenience method: returns *all* dashboard charts in one call.
    async fn get_dashboard_data(&self, input: DashboardInput) -> Result<DashboardData>;

    /// Daily sales timeline.
    async fn get_sales_over_time(&self, input: DateRangeInput) -> Result<Vec<DailySales>>;

    /// Top-selling products.
    async fn get_top_products(&self, input: TopProductsInput) -> Result<Vec<TopProduct>>;

    /// Inventory health snapshot.
    async fn get_inventory_summary(&self, store_id: Option<Uuid>) -> Result<InventorySummary>;

    /// Sales broken down by store.
    async fn get_sales_by_store(&self, input: DateRangeInput) -> Result<Vec<StoreSales>>;

    /// Sales broken down by payment method.
    async fn get_sales_by_payment(&self, input: DateRangeInput) -> Result<Vec<PaymentMethodSales>>;
}

// ---------------------------------------------------------------------------
// Interactor
// ---------------------------------------------------------------------------

pub struct ForAnalyticsInteractor<R: AnalyticsRepository> {
    repo: R,
}

impl<R: AnalyticsRepository> ForAnalyticsInteractor<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Parse an optional store_id string into `Option<Uuid>`.
    fn parse_store_id(store_id: &Option<String>) -> Result<Option<Uuid>> {
        match store_id {
            Some(s) if !s.is_empty() => Ok(Some(Uuid::parse_str(s)?)),
            _ => Ok(None),
        }
    }
}

#[async_trait]
impl<R: AnalyticsRepository + Sync> ForAnalyticsUseCases for ForAnalyticsInteractor<R> {
    async fn get_dashboard_data(&self, input: DashboardInput) -> Result<DashboardData> {
        let days = if input.days == 0 { 30 } else { input.days };
        let end = Utc::now();
        let start = end - Days::new(days as u64);

        let store_id = Self::parse_store_id(&input.store_id)?;

        let sales_over_time = self.repo.get_daily_sales(store_id, start, end).await?;
        let top_products = self
            .repo
            .get_top_products(store_id, start, end, 5)
            .await?;
        let inventory_summary = self.repo.get_inventory_summary(store_id).await?;
        let sales_by_store = self.repo.get_sales_by_store(start, end).await?;
        let sales_by_payment = self
            .repo
            .get_sales_by_payment_method(store_id, start, end)
            .await?;

        Ok(DashboardData {
            sales_over_time,
            top_products,
            inventory_summary,
            sales_by_store,
            sales_by_payment,
            period_start: start,
            period_end: end,
        })
    }

    async fn get_sales_over_time(&self, input: DateRangeInput) -> Result<Vec<DailySales>> {
        let store_id = Self::parse_store_id(&input.store_id)?;
        let start = DateTime::parse_from_rfc3339(&input.start)?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&input.end)?.with_timezone(&Utc);
        self.repo.get_daily_sales(store_id, start, end).await
    }

    async fn get_top_products(&self, input: TopProductsInput) -> Result<Vec<TopProduct>> {
        let store_id = Self::parse_store_id(&input.store_id)?;
        let start = DateTime::parse_from_rfc3339(&input.start)?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&input.end)?.with_timezone(&Utc);
        let limit = if input.limit == 0 { 5 } else { input.limit };
        self.repo.get_top_products(store_id, start, end, limit).await
    }

    async fn get_inventory_summary(&self, store_id: Option<Uuid>) -> Result<InventorySummary> {
        self.repo.get_inventory_summary(store_id).await
    }

    async fn get_sales_by_store(&self, input: DateRangeInput) -> Result<Vec<StoreSales>> {
        let start = DateTime::parse_from_rfc3339(&input.start)?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&input.end)?.with_timezone(&Utc);
        self.repo.get_sales_by_store(start, end).await
    }

    async fn get_sales_by_payment(&self, input: DateRangeInput) -> Result<Vec<PaymentMethodSales>> {
        let store_id = Self::parse_store_id(&input.store_id)?;
        let start = DateTime::parse_from_rfc3339(&input.start)?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&input.end)?.with_timezone(&Utc);
        self.repo
            .get_sales_by_payment_method(store_id, start, end)
            .await
    }
}
