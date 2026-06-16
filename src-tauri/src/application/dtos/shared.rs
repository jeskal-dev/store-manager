use crate::shared::criteria::{Criteria, FilterItem, Operator, Pagination, Sort, SortOrder};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaginationInput {
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: u32,

    #[validate(range(min = 1, max = 100, message = "Size must be between 1 and 100"))]
    pub size: u32,
}

impl TryInto<Pagination> for PaginationInput {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Pagination, Self::Error> {
        self.validate()?;
        Ok(Pagination {
            page: self.page,
            size: self.size,
        })
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FilterItemInput {
    #[validate(length(min = 1, message = "Filter field cannot be empty"))]
    pub field: String,
    pub operator: Operator,
    pub value: serde_json::Value,
}

impl TryInto<FilterItem<()>> for FilterItemInput {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<FilterItem<()>, Self::Error> {
        self.validate()?;
        Ok(FilterItem::new(self.field, self.operator, self.value))
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SortInput {
    #[validate(length(min = 1, message = "Sort field cannot be empty"))]
    pub field: String,
    pub order: SortOrder,
}

impl TryInto<Sort> for SortInput {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Sort, Self::Error> {
        self.validate()?;
        Ok(Sort {
            field: self.field,
            order: self.order,
        })
    }
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct CriteriaInput {
    #[validate(nested)] // Valida el struct interno si está presente
    pub pagination: Option<PaginationInput>,
    #[validate(nested)] // Valida cada FilterItemInput dentro del Vec
    pub filters: Option<Vec<FilterItemInput>>,
    #[validate(nested)]
    pub global_filter: Option<Vec<FilterItemInput>>,
    #[validate(nested)] // Valida cada SortInput dentro del Vec
    pub sort: Option<Vec<SortInput>>,
}

impl TryInto<Criteria<()>> for CriteriaInput {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Criteria<()>, Self::Error> {
        self.validate()?;

        let mut criteria = Criteria::new();
        criteria.pagination = self.pagination.map(|v| v.try_into()).transpose()?;
        criteria.filters = self
            .filters
            .map(|arr| {
                arr.into_iter()
                    .map(|v| v.try_into())
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        criteria.global_filters = self
            .global_filter
            .map(|arr| {
                arr.into_iter()
                    .map(|v| v.try_into())
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        criteria.sort = self
            .sort
            .map(|arr| {
                arr.into_iter()
                    .map(|v| v.try_into())
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        Ok(criteria)
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ByIDInput {
    #[validate(length(min = 1, message = "ID cannot be empty"))]
    pub id: String,
}

impl TryInto<Uuid> for ByIDInput {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Uuid, Self::Error> {
        self.validate()?;
        Ok(Uuid::parse_str(&self.id)?.into())
    }
}
