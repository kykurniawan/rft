#[derive(Debug, Clone)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct Sort {
    pub by: String,
    pub order: SortOrder,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub by: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub pagination: Option<Pagination>,
    pub search: Option<String>,
    pub sorts: Option<Vec<Sort>>,
    pub filters: Option<Vec<Filter>>,
}
