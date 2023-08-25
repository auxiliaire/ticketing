use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Page<T> {
    pub total: i64,
    pub offset: u64,
    pub limit: u64,
    pub list: Vec<T>,
}
