use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(rename_all = "lowercase")]
pub enum Operator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Like,
    Ilike,
    In,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sort {
    pub field: String,
    pub order: SortOrder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterItem<T> {
    pub field: String,
    pub operator: Operator,
    pub value: serde_json::Value,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T> Clone for FilterItem<T> {
    fn clone(&self) -> Self {
        Self {
            field: self.field.clone(),
            operator: self.operator.clone(),
            value: self.value.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> FilterItem<T> {
    pub fn new(
        field: impl Into<String>,
        operator: Operator,
        value: impl Into<serde_json::Value>,
    ) -> Self {
        Self {
            field: field.into(),
            operator,
            value: value.into(),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Criteria<T> {
    pub pagination: Option<Pagination>,
    pub filters: Option<Vec<FilterItem<T>>>,
    pub global_filters: Option<Vec<FilterItem<T>>>,
    pub sort: Option<Vec<Sort>>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T> Criteria<T> {
    pub fn new() -> Self {
        Self {
            pagination: None,
            filters: None,
            global_filters: None,
            sort: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_pagination(mut self, page: u32, size: u32) -> Self {
        self.pagination = Some(Pagination { page, size });
        self
    }

    pub fn with_filters(mut self, filters: Vec<FilterItem<T>>) -> Self {
        self.filters = Some(filters);
        self
    }

    pub fn with_global_filters(mut self, global_filters: Vec<FilterItem<T>>) -> Self {
        self.global_filters = Some(global_filters);
        self
    }

    pub fn with_sort(mut self, sort: Vec<Sort>) -> Self {
        self.sort = Some(sort);
        self
    }
}

impl<T> Default for Criteria<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for Criteria<T> {
    fn clone(&self) -> Self {
        Self {
            pagination: self.pagination.clone(),
            filters: self.filters.clone(),
            global_filters: self.global_filters.clone(),
            sort: self.sort.clone(),
            _phantom: PhantomData,
        }
    }
}
