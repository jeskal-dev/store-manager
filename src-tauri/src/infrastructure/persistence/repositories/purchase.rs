use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use sqlx::SqlitePool;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::aggregates::purchase::Purchase;
use crate::domain::entities::purchase_item::PurchaseItem;
use crate::domain::repositories::purchase::PurchaseRepository;
use crate::domain::repositories::Repository;
use crate::domain::value_objects::common::{Code, Money, TextValue};
use crate::shared::criteria::{Criteria, PaginatedResult, PaginationMeta};

use super::search_builder::{push_filter_condition, push_filter_value, push_sort};

#[derive(FromRow)]
struct PurchaseRow {
    id: Uuid,
    supplier_id: Option<Uuid>,
    store_id: Uuid,
    total_cost: Money,
    purchase_date: DateTime<Utc>,
    description: Option<TextValue>,
    purchase_code: Code,
}

impl From<PurchaseRow> for Purchase {
    fn from(r: PurchaseRow) -> Self {
        Self {
            id: r.id,
            supplier_id: r.supplier_id,
            store_id: r.store_id,
            total_cost: r.total_cost,
            purchase_date: r.purchase_date,
            description: r.description,
            purchase_code: r.purchase_code,
            items: Vec::new(),
        }
    }
}

pub struct SqlitePurchaseRepository {
    pool: SqlitePool,
}

impl SqlitePurchaseRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn to_purchase(row: PurchaseRow, items: Vec<PurchaseItem>) -> Purchase {
        Purchase {
            id: row.id,
            supplier_id: row.supplier_id,
            store_id: row.store_id,
            total_cost: row.total_cost,
            purchase_date: row.purchase_date,
            description: row.description,
            purchase_code: row.purchase_code,
            items,
        }
    }

    async fn load_items(&self, purchase_id: Uuid) -> Result<Vec<PurchaseItem>> {
        sqlx::query_as::<_, PurchaseItem>("SELECT * FROM purchase_items WHERE purchase_id = ?")
            .bind(purchase_id)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }
}

#[async_trait]
impl Repository<Purchase> for SqlitePurchaseRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Purchase>> {
        let row = sqlx::query_as::<_, PurchaseRow>("SELECT * FROM purchases WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => {
                let items = self.load_items(id).await?;
                Ok(Some(Self::to_purchase(r, items)))
            }
            None => Ok(None),
        }
    }

    async fn search(&self, criteria: Criteria<Purchase>) -> Result<PaginatedResult<Purchase>> {
        let mut count_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM purchases WHERE 1=1");
        let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM purchases WHERE 1=1");

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

        let data: Vec<PurchaseRow> = query_builder.build_query_as().fetch_all(&self.pool).await?;
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

    async fn create(&self, entity: &Purchase) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO purchases (id, supplier_id, store_id, total_cost, purchase_date, description, purchase_code) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id)
        .bind(entity.supplier_id)
        .bind(entity.store_id)
        .bind(&entity.total_cost)
        .bind(entity.purchase_date)
        .bind(&entity.description)
        .bind(&entity.purchase_code)
        .execute(&mut *tx)
        .await?;

        for item in &entity.items {
            sqlx::query(
                "INSERT INTO purchase_items (id, purchase_id, inventory_id, quantity, unit_cost, subtotal) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(item.id)
            .bind(item.purchase_id)
            .bind(item.inventory_id)
            .bind(&item.quantity)
            .bind(&item.unit_cost)
            .bind(&item.subtotal)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn update(&self, entity: &Purchase) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "UPDATE purchases SET supplier_id = ?, store_id = ?, total_cost = ?, purchase_date = ?, description = ?, purchase_code = ? WHERE id = ?",
        )
        .bind(entity.supplier_id)
        .bind(entity.store_id)
        .bind(&entity.total_cost)
        .bind(entity.purchase_date)
        .bind(&entity.description)
        .bind(&entity.purchase_code)
        .bind(entity.id)
        .execute(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM purchase_items WHERE purchase_id = ?")
            .bind(entity.id)
            .execute(&mut *tx)
            .await?;

        for item in &entity.items {
            sqlx::query(
                "INSERT INTO purchase_items (id, purchase_id, inventory_id, quantity, unit_cost, subtotal) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(item.id)
            .bind(item.purchase_id)
            .bind(item.inventory_id)
            .bind(&item.quantity)
            .bind(&item.unit_cost)
            .bind(&item.subtotal)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("DELETE FROM purchase_items WHERE purchase_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM purchases WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl PurchaseRepository for SqlitePurchaseRepository {
    async fn find_by_code(&self, code: &str) -> Result<Option<Purchase>> {
        let row =
            sqlx::query_as::<_, PurchaseRow>("SELECT * FROM purchases WHERE purchase_code = ?")
                .bind(code)
                .fetch_optional(&self.pool)
                .await?;

        match row {
            Some(r) => {
                let items = self.load_items(r.id).await?;
                Ok(Some(Self::to_purchase(r, items)))
            }
            None => Ok(None),
        }
    }

    async fn find_by_store(&self, store_id: Uuid) -> Result<Vec<Purchase>> {
        let rows = sqlx::query_as::<_, PurchaseRow>("SELECT * FROM purchases WHERE store_id = ?")
            .bind(store_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_supplier(&self, supplier_id: Uuid) -> Result<Vec<Purchase>> {
        let rows =
            sqlx::query_as::<_, PurchaseRow>("SELECT * FROM purchases WHERE supplier_id = ?")
                .bind(supplier_id)
                .fetch_all(&self.pool)
                .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}
