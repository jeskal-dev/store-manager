//! Analytics use cases — read-only aggregated queries for dashboard charts.
//!
//! The interactor depends on an [`AnalyticsRepository`] trait that is defined
//! here (application port) and implemented in the infrastructure layer.

use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Days, Utc};
use uuid::Uuid;

use crate::application::ports::analytics::AnalyticsRepository;

use super::super::dtos::analytics::{
    DailySalesOutput, DashboardData, DashboardInput, DateRangeInput, InventorySummaryOutput,
    PaymentMethodSalesOutput, StoreSalesOutput, TopProductOutput, TopProductsInput,
};

// ---------------------------------------------------------------------------
// Use case trait
// ---------------------------------------------------------------------------

#[async_trait]
pub trait ForAnalyticsUseCases {
    /// Convenience method: returns *all* dashboard charts in one call.
    async fn get_dashboard_data(&self, input: DashboardInput) -> Result<DashboardData>;

    /// Daily sales timeline.
    async fn get_sales_over_time(&self, input: DateRangeInput) -> Result<Vec<DailySalesOutput>>;

    /// Top-selling products.
    async fn get_top_products(&self, input: TopProductsInput) -> Result<Vec<TopProductOutput>>;

    /// Inventory health snapshot.
    async fn get_inventory_summary(&self, store_id: Option<Uuid>)
        -> Result<InventorySummaryOutput>;

    /// Sales broken down by store.
    async fn get_sales_by_store(&self, input: DateRangeInput) -> Result<Vec<StoreSalesOutput>>;

    /// Sales broken down by payment method.
    async fn get_sales_by_payment(
        &self,
        input: DateRangeInput,
    ) -> Result<Vec<PaymentMethodSalesOutput>>;
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
}

#[async_trait]
impl<R: AnalyticsRepository + Sync> ForAnalyticsUseCases for ForAnalyticsInteractor<R> {
    async fn get_dashboard_data(&self, input: DashboardInput) -> Result<DashboardData> {
        let days = if input.days == 0 { 30 } else { input.days };
        let end = Utc::now();
        let start = end - Days::new(days as u64);

        let store_id = input.store_id.map(|id| Uuid::from_str(&id).unwrap());

        let sales_over_time = self.repo.get_daily_sales(store_id, start, end).await?;
        let top_products = self.repo.get_top_products(store_id, start, end, 5).await?;
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

    async fn get_sales_over_time(&self, input: DateRangeInput) -> Result<Vec<DailySalesOutput>> {
        let store_id = input.store_id.map(|id| Uuid::from_str(&id).unwrap());
        let start = DateTime::parse_from_rfc3339(&input.start)?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&input.end)?.with_timezone(&Utc);
        self.repo.get_daily_sales(store_id, start, end).await
    }

    async fn get_top_products(&self, input: TopProductsInput) -> Result<Vec<TopProductOutput>> {
        let store_id = input.store_id.map(|id| Uuid::from_str(&id).unwrap());
        let start = DateTime::parse_from_rfc3339(&input.start)?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&input.end)?.with_timezone(&Utc);
        let limit = if input.limit == 0 { 5 } else { input.limit };
        self.repo
            .get_top_products(store_id, start, end, limit)
            .await
    }

    async fn get_inventory_summary(
        &self,
        store_id: Option<Uuid>,
    ) -> Result<InventorySummaryOutput> {
        self.repo.get_inventory_summary(store_id).await
    }

    async fn get_sales_by_store(&self, input: DateRangeInput) -> Result<Vec<StoreSalesOutput>> {
        let start = DateTime::parse_from_rfc3339(&input.start)?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&input.end)?.with_timezone(&Utc);
        self.repo.get_sales_by_store(start, end).await
    }

    async fn get_sales_by_payment(
        &self,
        input: DateRangeInput,
    ) -> Result<Vec<PaymentMethodSalesOutput>> {
        let store_id = input.store_id.map(|id| Uuid::from_str(&id).unwrap());
        let start = DateTime::parse_from_rfc3339(&input.start)?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&input.end)?.with_timezone(&Utc);
        self.repo
            .get_sales_by_payment_method(store_id, start, end)
            .await
    }
}
