//! SQLite implementation of the analytics repository.
//!
//! All queries are read-only aggregations on existing tables.
//! No new tables or migrations required.
//!
//! Monetary values (`Money` type) are stored as TEXT in SQLite, so we
//! `CAST(... AS REAL)` before any arithmetic, then `CAST(... AS TEXT)` for
//! transport back to Rust where we parse into `Decimal`.

use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::application::{
    dtos::analytics::{
        DailySalesOutput, InventorySummaryOutput, PaymentMethodSalesOutput, StoreSalesOutput,
        TopProductOutput,
    },
    ports::analytics::AnalyticsRepository,
};

use super::row_types::{
    DailySalesRow, InventorySummaryRow, PaymentMethodSalesRow, StoreSalesRow, TopProductRow,
};

// ---------------------------------------------------------------------------
// Repository
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct SqliteAnalyticsRepository {
    pool: SqlitePool,
}

impl SqliteAnalyticsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AnalyticsRepository for SqliteAnalyticsRepository {
    // -----------------------------------------------------------------------
    // Daily sales
    // -----------------------------------------------------------------------
    async fn get_daily_sales(
        &self,
        store_id: Option<Uuid>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<DailySalesOutput>> {
        // Money is stored as TEXT, so CAST each row to REAL before SUM.
        let sql = r#"
            SELECT DATE(sale_date)                                     AS date,
                   CAST(SUM(CAST(total AS REAL)) AS TEXT)              AS total,
                   COUNT(*)                                            AS count
            FROM sales
            WHERE sale_date >= ?1 AND sale_date <= ?2
              AND (?3 IS NULL OR store_id = ?3)
            GROUP BY DATE(sale_date)
            ORDER BY date ASC
        "#;

        // sqlite treats NULL as a separate value for bind params; we pass
        // store_id as-is (Option<Uuid>) and sqlx handles the NULL.
        let rows = sqlx::query_as::<_, DailySalesRow>(sql)
            .bind(start)
            .bind(end)
            .bind(store_id)
            .fetch_all(&self.pool)
            .await?;

        let data: Vec<DailySalesOutput> = rows
            .into_iter()
            .map(|r| DailySalesOutput {
                date: r.date.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                total: Decimal::from_str(&r.total).unwrap_or(Decimal::ZERO),
                count: r.count,
            })
            .collect();

        Ok(data)
    }

    // -----------------------------------------------------------------------
    // Top products by revenue
    // -----------------------------------------------------------------------
    async fn get_top_products(
        &self,
        store_id: Option<Uuid>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        limit: u32,
    ) -> Result<Vec<TopProductOutput>> {
        let sql = r#"
            SELECT p.id                                                AS product_id,
                   p.product_code,
                   p.name,
                   CAST(SUM(CAST(si.subtotal AS REAL)) AS TEXT)        AS total_revenue,
                   SUM(si.quantity)                                    AS units_sold
            FROM sale_items si
            JOIN sales s     ON si.sale_id = s.id
            JOIN inventory i ON si.inventory_id = i.id
            JOIN products p  ON i.product_id = p.id
            WHERE s.sale_date >= ?1 AND s.sale_date <= ?2
              AND (?3 IS NULL OR s.store_id = ?3)
            GROUP BY p.id, p.product_code, p.name
            ORDER BY SUM(CAST(si.subtotal AS REAL)) DESC
            LIMIT ?4
        "#;

        let rows = sqlx::query_as::<_, TopProductRow>(sql)
            .bind(start)
            .bind(end)
            .bind(store_id)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let data: Vec<TopProductOutput> = rows
            .into_iter()
            .map(|r| TopProductOutput {
                product_id: r.product_id,
                product_code: r.product_code,
                name: r.name,
                total_revenue: Decimal::from_str(&r.total_revenue).unwrap_or(Decimal::ZERO),
                units_sold: r.units_sold,
            })
            .collect();

        Ok(data)
    }

    // -----------------------------------------------------------------------
    // Inventory summary
    // -----------------------------------------------------------------------
    async fn get_inventory_summary(
        &self,
        store_id: Option<Uuid>,
    ) -> Result<InventorySummaryOutput> {
        // quantity is INTEGER (Quantity -> i32), price_local is TEXT (Money).
        let sql = r#"
            SELECT COUNT(*)                                                                 AS total_products,
                   COALESCE(SUM(CASE WHEN quantity <= min_stock AND quantity > 0 THEN 1 ELSE 0 END), 0) AS low_stock,
                   COALESCE(SUM(CASE WHEN quantity <= 0 THEN 1 ELSE 0 END), 0)              AS out_of_stock,
                   CAST(COALESCE(SUM(CAST(quantity AS REAL) * CAST(price_local AS REAL)), 0) AS TEXT) AS total_value
            FROM inventory
            WHERE active = 1
              AND (?1 IS NULL OR store_id = ?1)
        "#;

        let row = sqlx::query_as::<_, InventorySummaryRow>(sql)
            .bind(store_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(InventorySummaryOutput {
            total_products: row.total_products,
            low_stock: row.low_stock,
            out_of_stock: row.out_of_stock,
            total_value: Decimal::from_str(&row.total_value).unwrap_or(Decimal::ZERO),
        })
    }

    // -----------------------------------------------------------------------
    // Sales by store
    // -----------------------------------------------------------------------
    async fn get_sales_by_store(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<StoreSalesOutput>> {
        let sql = r#"
            SELECT st.id                                              AS store_id,
                   st.store_code,
                   st.name,
                   CAST(SUM(CAST(s.total AS REAL)) AS TEXT)           AS total_revenue,
                   COUNT(s.id)                                        AS sales_count
            FROM sales s
            JOIN stores st ON s.store_id = st.id
            WHERE s.sale_date >= ?1 AND s.sale_date <= ?2
            GROUP BY st.id, st.store_code, st.name
            ORDER BY SUM(CAST(s.total AS REAL)) DESC
        "#;

        let rows = sqlx::query_as::<_, StoreSalesRow>(sql)
            .bind(start)
            .bind(end)
            .fetch_all(&self.pool)
            .await?;

        let data: Vec<StoreSalesOutput> = rows
            .into_iter()
            .map(|r| StoreSalesOutput {
                store_id: r.store_id,
                store_code: r.store_code,
                name: r.name,
                total_revenue: Decimal::from_str(&r.total_revenue).unwrap_or(Decimal::ZERO),
                sales_count: r.sales_count,
            })
            .collect();

        Ok(data)
    }

    // -----------------------------------------------------------------------
    // Sales by payment method
    // -----------------------------------------------------------------------
    async fn get_sales_by_payment_method(
        &self,
        store_id: Option<Uuid>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<PaymentMethodSalesOutput>> {
        let sql = r#"
            SELECT payment_method                                     AS method,
                   CAST(SUM(CAST(total AS REAL)) AS TEXT)             AS total_revenue,
                   COUNT(*)                                           AS count
            FROM sales
            WHERE sale_date >= ?1 AND sale_date <= ?2
              AND (?3 IS NULL OR store_id = ?3)
            GROUP BY payment_method
            ORDER BY SUM(CAST(total AS REAL)) DESC
        "#;

        let rows = sqlx::query_as::<_, PaymentMethodSalesRow>(sql)
            .bind(start)
            .bind(end)
            .bind(store_id)
            .fetch_all(&self.pool)
            .await?;

        let data: Vec<PaymentMethodSalesOutput> = rows
            .into_iter()
            .map(|r| PaymentMethodSalesOutput {
                method: r.method,
                total_revenue: Decimal::from_str(&r.total_revenue).unwrap_or(Decimal::ZERO),
                count: r.count,
            })
            .collect();

        Ok(data)
    }
}
