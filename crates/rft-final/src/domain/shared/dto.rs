use serde::{Deserialize, Serialize};

use crate::domain::shared::db::{Filter, Pagination, Query, Sort, SortOrder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,

    #[serde(default)]
    pub sort: Vec<String>,

    #[serde(default)]
    pub filter: Vec<String>,
}

impl PaginationRequest {
    pub fn into_query(&self) -> Query {
        let sorts = self
            .sort
            .iter()
            .filter_map(|item| {
                let (by, order) = item.split_once(':')?;

                Some(Sort {
                    by: by.to_string(),
                    order: match order {
                        "desc" => SortOrder::Desc,
                        "asc" => SortOrder::Asc,
                        _ => SortOrder::Asc,
                    },
                })
            })
            .collect::<Vec<_>>();

        let filters = self
            .filter
            .iter()
            .filter_map(|item| {
                let (by, value) = item.split_once(':')?;

                Some(Filter {
                    by: by.to_string(),
                    value: value.to_string(),
                })
            })
            .collect::<Vec<_>>();

        Query {
            pagination: Some(Pagination {
                page: self.page.unwrap_or(1),
                page_size: self.page_size.unwrap_or(10),
            }),
            search: self.search.clone(),
            sorts: Some(sorts),
            filters: Some(filters),
        }
    }
}
