use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default)]
    pub offset: Option<u64>,
    #[serde(default)]
    pub limit: Option<u64>,
}
