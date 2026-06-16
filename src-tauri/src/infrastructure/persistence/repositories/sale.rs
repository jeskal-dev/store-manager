use std::str::FromStr;

use async_trait::async_trait;
use rust_decimal::Decimal;
use sqlx::SqlitePool;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::aggregates::sale::Sale;
use crate::domain::entities::sale_item::SaleItem;
use crate::domain::repositories::sale::SalesRepository;
use crate::domain::repositories::Repository;
use crate::domain::value_objects::sale::PaymentMethod;
use crate::shared::criteria::{Criteria, PaginatedResult, PaginationMeta};

use super::row_types::SalesRow;
use super::search_builder::{push_filter_condition, push_filter_value, push_sort};

#[derive(Clone)]
pub struct SqliteSalesRepository {
    pool: SqlitePool,
}

impl SqliteSalesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn load_items(&self, sale_id: Uuid) -> Result<Vec<SaleItem>> {
        sqlx::query_as::<_, SaleItem>("SELECT * FROM sale_items WHERE sale_id = ?")
            .bind(sale_id)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }
}

#[async_trait]
impl Repository<Sale> for SqliteSalesRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Sale>> {
        let row = sqlx::query_as::<_, SalesRow>("SELECT * FROM sales WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => {
                let items = self.load_items(id).await?;
                Ok(Some(Sale::restore(
                    r.id,
                    r.store_id,
                    r.sale_code,
                    Decimal::from_str(&r.total).unwrap_or(Decimal::ZERO),
                    PaymentMethod::from_str(&r.payment_method)?,
                    r.sale_date,
                    items,
                )?))
            }
            None => Ok(None),
        }
    }

    async fn search(&self, criteria: Criteria<Sale>) -> Result<PaginatedResult<Sale>> {
        let mut count_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM sales WHERE 1=1");
        let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM sales WHERE 1=1");

        if let Some(filters) = &criteria.filters {
            for filter in filters {
                count_builder.push(" AND ");
                push_filter_condition(&mut count_builder, &filter.field, &filter.operator);
                push_filter_value(&mut count_builder, &filter.value, &filter.operator);

                query_builder.push(" AND ");
                push_filter_condition(&mut query_builder, &filter.field, &filter.operator);
                push_filter_value(&mut query_builder, &filter.value, &filter.operator);
            }
        }

        if let Some(global) = &criteria.global_filters {
            if !global.is_empty() {
                count_builder.push(" AND (");
                query_builder.push(" AND (");
                for (i, filter) in global.iter().enumerate() {
                    if i > 0 {
                        count_builder.push(" OR ");
                        query_builder.push(" OR ");
                    }
                    push_filter_condition(&mut count_builder, &filter.field, &filter.operator);
                    push_filter_value(&mut count_builder, &filter.value, &filter.operator);

                    push_filter_condition(&mut query_builder, &filter.field, &filter.operator);
                    push_filter_value(&mut query_builder, &filter.value, &filter.operator);
                }
                count_builder.push(")");
                query_builder.push(")");
            }
        }

        let total_items: (i64,) = count_builder.build_query_as().fetch_one(&self.pool).await?;
        let total_items = total_items.0 as u64;

        if let Some(sort) = &criteria.sort {
            push_sort(&mut query_builder, sort);
        }

        if let Some(pagination) = &criteria.pagination {
            let offset = (pagination.page.saturating_sub(1)) * pagination.size;
            query_builder.push(" LIMIT ");
            query_builder.push_bind(pagination.size as i64);
            query_builder.push(" OFFSET ");
            query_builder.push_bind(offset as i64);
        }

        let data: Vec<SalesRow> = query_builder.build_query_as().fetch_all(&self.pool).await?;
        let data = data.into_iter().map(Into::into).collect();

        let total_pages = match &criteria.pagination {
            Some(p) => {
                if total_items == 0 {
                    1
                } else {
                    ((total_items as f64) / (p.size as f64)).ceil() as u32
                }
            }
            None => 1,
        };
        let current_page = criteria.pagination.as_ref().map(|p| p.page).unwrap_or(1);

        Ok(PaginatedResult {
            data,
            meta: PaginationMeta {
                current_page,
                total_pages,
                total_items,
            },
        })
    }

    async fn create(&self, entity: &Sale) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO sales (id, store_id, sale_code, total, payment_method, sale_date) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id)
        .bind(entity.store_id)
        .bind(&entity.sale_code)
        .bind(&entity.total)
        .bind(&entity.payment_method)
        .bind(entity.sale_date)
        .execute(&mut *tx)
        .await?;

        for item in &entity.items {
            sqlx::query(
                "INSERT INTO sale_items (id, sale_id, inventory_id, quantity, unit_price, subtotal) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(item.id)
            .bind(item.sale_id)
            .bind(item.inventory_id)
            .bind(&item.quantity)
            .bind(&item.unit_price)
            .bind(&item.subtotal)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn update(&self, entity: &Sale) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "UPDATE sales SET store_id = ?, sale_code = ?, total = ?, payment_method = ?, sale_date = ? WHERE id = ?",
        )
        .bind(entity.store_id)
        .bind(&entity.sale_code)
        .bind(&entity.total)
        .bind(&entity.payment_method)
        .bind(entity.sale_date)
        .bind(entity.id)
        .execute(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM sale_items WHERE sale_id = ?")
            .bind(entity.id)
            .execute(&mut *tx)
            .await?;

        for item in &entity.items {
            sqlx::query(
                "INSERT INTO sale_items (id, sale_id, inventory_id, quantity, unit_price, subtotal) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(item.id)
            .bind(item.sale_id)
            .bind(item.inventory_id)
            .bind(&item.quantity)
            .bind(&item.unit_price)
            .bind(&item.subtotal)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("DELETE FROM sale_items WHERE sale_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM sales WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn delete_many(&self, ids: &[Uuid]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for id in ids {
            sqlx::query("DELETE FROM sale_items WHERE sale_id = ?")
                .bind(id)
                .execute(&mut *tx)
                .await?;

            sqlx::query("DELETE FROM sales WHERE id = ?")
                .bind(id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl SalesRepository for SqliteSalesRepository {
    async fn find_by_code(&self, code: &str) -> Result<Option<Sale>> {
        let row = sqlx::query_as::<_, SalesRow>("SELECT * FROM sales WHERE sale_code = ?")
            .bind(code)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => {
                let items = self.load_items(r.id).await?;
                Ok(Some(Sale::restore(
                    r.id,
                    r.store_id,
                    r.sale_code,
                    Decimal::from_str(&r.total).unwrap_or(Decimal::ZERO),
                    PaymentMethod::from_str(&r.payment_method)?,
                    r.sale_date,
                    items,
                )?))
            }
            None => Ok(None),
        }
    }

    async fn find_by_store(&self, store_id: Uuid) -> Result<Vec<Sale>> {
        let rows = sqlx::query_as::<_, SalesRow>("SELECT * FROM sales WHERE store_id = ?")
            .bind(store_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}
