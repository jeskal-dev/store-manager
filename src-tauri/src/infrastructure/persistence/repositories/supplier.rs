use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::entities::supplier::Supplier;
use crate::domain::repositories::supplier::SupplierRepository;
use crate::domain::repositories::Repository;
use crate::shared::criteria::{Criteria, PaginatedResult, PaginationMeta};

use super::search_builder::{push_filter_condition, push_filter_value, push_sort};

#[derive(Clone)]
pub struct SqliteSupplierRepository {
    pool: SqlitePool,
}

impl SqliteSupplierRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<Supplier> for SqliteSupplierRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Supplier>> {
        sqlx::query_as::<_, Supplier>("SELECT * FROM suppliers WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn search(&self, criteria: Criteria<Supplier>) -> Result<PaginatedResult<Supplier>> {
        let mut count_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM suppliers WHERE 1=1");
        let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM suppliers WHERE 1=1");

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
            .build_query_as::<Supplier>()
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

    async fn create(&self, entity: &Supplier) -> Result<()> {
        sqlx::query(
            "INSERT INTO suppliers (id, name, supplier_code, contact_name, phone, address, active) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id)
        .bind(&entity.name)
        .bind(&entity.supplier_code)
        .bind(&entity.contact_name)
        .bind(&entity.phone)
        .bind(&entity.address)
        .bind(entity.active)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, entity: &Supplier) -> Result<()> {
        sqlx::query(
            "UPDATE suppliers SET name = ?, supplier_code = ?, contact_name = ?, phone = ?, address = ?, active = ? WHERE id = ?",
        )
        .bind(&entity.name)
        .bind(&entity.supplier_code)
        .bind(&entity.contact_name)
        .bind(&entity.phone)
        .bind(&entity.address)
        .bind(entity.active)
        .bind(entity.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM suppliers WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_many(&self, ids: &[Uuid]) -> Result<()> {
        let mut qb = sqlx::QueryBuilder::new("DELETE FROM suppliers WHERE id IN (");
        let mut first = true;
        for id in ids {
            if !first {
                qb.push(", ");
            }
            qb.push_bind(id);
            first = false;
        }
        qb.push(")");
        qb.build().execute(&self.pool).await?;
        Ok(())
    }
}

#[async_trait]
impl SupplierRepository for SqliteSupplierRepository {
    async fn find_by_code(&self, code: &str) -> Result<Option<Supplier>> {
        sqlx::query_as::<_, Supplier>("SELECT * FROM suppliers WHERE supplier_code = ?")
            .bind(code)
            .fetch_optional(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn exists_by_code(&self, code: &str) -> Result<bool> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM suppliers WHERE supplier_code = ?")
            .bind(code)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0 > 0)
    }
}
