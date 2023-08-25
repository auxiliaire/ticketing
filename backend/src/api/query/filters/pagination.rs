use crate::api::consts::{DEFAULT_PAGINATION_LIMIT, DEFAULT_PAGINATION_OFFSET};
use sea_orm::FromQueryResult;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default = "default_offset")]
    pub offset: Option<u64>,
    #[serde(default = "default_limit")]
    pub limit: Option<u64>,
}

fn default_offset() -> Option<u64> {
    Some(DEFAULT_PAGINATION_OFFSET)
}

fn default_limit() -> Option<u64> {
    Some(DEFAULT_PAGINATION_LIMIT)
}

#[derive(FromQueryResult)]
pub struct TotalCount {
    pub count: i64,
}
