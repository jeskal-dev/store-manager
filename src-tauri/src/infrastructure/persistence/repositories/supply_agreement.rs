use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use anyhow::Result;

use crate::domain::entities::supply_agreement::SupplyAgreement;
use crate::domain::repositories::supply_agreement::SupplyAgreementRepository;
use crate::domain::repositories::Repository;
use crate::shared::criteria::{Criteria, PaginatedResult, PaginationMeta};

use super::search_builder::{push_filter_condition, push_filter_value, push_sort};

pub struct SqliteSupplyAgreementRepository {
    pool: SqlitePool,
}

impl SqliteSupplyAgreementRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<SupplyAgreement> for SqliteSupplyAgreementRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SupplyAgreement>> {
        sqlx::query_as::<_, SupplyAgreement>("SELECT * FROM supply_agreements WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn search(
        &self,
        criteria: Criteria<SupplyAgreement>,
    ) -> Result<PaginatedResult<SupplyAgreement>> {
        let mut count_builder =
            sqlx::QueryBuilder::new("SELECT COUNT(*) FROM supply_agreements WHERE 1=1");
        let mut query_builder =
            sqlx::QueryBuilder::new("SELECT * FROM supply_agreements WHERE 1=1");

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
            .build_query_as::<SupplyAgreement>()
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

    async fn create(&self, entity: &SupplyAgreement) -> Result<()> {
        sqlx::query(
            "INSERT INTO supply_agreements (id, product_id, supplier_id, cost, active) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(entity.id)
        .bind(entity.product_id)
        .bind(entity.supplier_id)
        .bind(&entity.cost)
        .bind(entity.active)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, entity: &SupplyAgreement) -> Result<()> {
        sqlx::query(
            "UPDATE supply_agreements SET product_id = ?, supplier_id = ?, cost = ?, active = ? WHERE id = ?",
        )
        .bind(entity.product_id)
        .bind(entity.supplier_id)
        .bind(&entity.cost)
        .bind(entity.active)
        .bind(entity.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM supply_agreements WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl SupplyAgreementRepository for SqliteSupplyAgreementRepository {
    async fn find_by_product(&self, product_id: Uuid) -> Result<Vec<SupplyAgreement>> {
        sqlx::query_as::<_, SupplyAgreement>("SELECT * FROM supply_agreements WHERE product_id = ?")
            .bind(product_id)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn find_by_supplier(&self, supplier_id: Uuid) -> Result<Vec<SupplyAgreement>> {
        sqlx::query_as::<_, SupplyAgreement>(
            "SELECT * FROM supply_agreements WHERE supplier_id = ?",
        )
        .bind(supplier_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
}
