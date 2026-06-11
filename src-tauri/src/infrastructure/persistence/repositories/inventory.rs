use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::entities::inventory::Inventory;
use crate::domain::repositories::inventory::InventoryRepository;
use crate::domain::repositories::Repository;
use crate::shared::criteria::{Criteria, PaginatedResult, PaginationMeta};

use super::search_builder::{push_filter_condition, push_filter_value, push_sort};

pub struct SqliteInventoryRepository {
    pool: SqlitePool,
}

impl SqliteInventoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<Inventory> for SqliteInventoryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Inventory>> {
        sqlx::query_as::<_, Inventory>("SELECT * FROM inventory WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn search(&self, criteria: Criteria<Inventory>) -> Result<PaginatedResult<Inventory>> {
        let mut count_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM inventory WHERE 1=1");
        let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM inventory WHERE 1=1");

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

        let data = query_builder
            .build_query_as::<Inventory>()
            .fetch_all(&self.pool)
            .await?;

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

    async fn create(&self, entity: &Inventory) -> Result<()> {
        sqlx::query(
            "INSERT INTO inventory (id, inventory_code, store_id, product_id, quantity, price_local, min_stock, status, last_updated, active) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id)
        .bind(&entity.inventory_code)
        .bind(entity.store_id)
        .bind(entity.product_id)
        .bind(&entity.quantity)
        .bind(&entity.price_local)
        .bind(&entity.min_stock)
        .bind(&entity.status)
        .bind(entity.last_updated)
        .bind(entity.active)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, entity: &Inventory) -> Result<()> {
        sqlx::query(
            "UPDATE inventory SET inventory_code = ?, store_id = ?, product_id = ?, quantity = ?, price_local = ?, min_stock = ?, status = ?, last_updated = ?, active = ? WHERE id = ?",
        )
        .bind(&entity.inventory_code)
        .bind(entity.store_id)
        .bind(entity.product_id)
        .bind(&entity.quantity)
        .bind(&entity.price_local)
        .bind(&entity.min_stock)
        .bind(&entity.status)
        .bind(entity.last_updated)
        .bind(entity.active)
        .bind(entity.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM inventory WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl InventoryRepository for SqliteInventoryRepository {
    async fn find_by_code(&self, code: &str) -> Result<Option<Inventory>> {
        sqlx::query_as::<_, Inventory>("SELECT * FROM inventory WHERE inventory_code = ?")
            .bind(code)
            .fetch_optional(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn find_by_store(&self, store_id: Uuid) -> Result<Vec<Inventory>> {
        sqlx::query_as::<_, Inventory>("SELECT * FROM inventory WHERE store_id = ?")
            .bind(store_id)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn find_by_product(&self, product_id: Uuid) -> Result<Vec<Inventory>> {
        sqlx::query_as::<_, Inventory>("SELECT * FROM inventory WHERE product_id = ?")
            .bind(product_id)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn find_by_status(&self, status: &str) -> Result<Vec<Inventory>> {
        sqlx::query_as::<_, Inventory>("SELECT * FROM inventory WHERE status = ?")
            .bind(status)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn find_low_stock(&self, store_id: Option<Uuid>) -> Result<Vec<Inventory>> {
        match store_id {
            Some(sid) => sqlx::query_as::<_, Inventory>(
                "SELECT * FROM inventory WHERE quantity <= min_stock AND store_id = ?",
            )
            .bind(sid)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into),
            None => sqlx::query_as::<_, Inventory>(
                "SELECT * FROM inventory WHERE quantity <= min_stock",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into),
        }
    }
}
