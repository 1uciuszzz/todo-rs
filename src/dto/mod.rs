use serde::{Deserialize, Serialize};

pub mod todo;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationResult<T> {
    datalist: T,
    total: u64,
}

impl<T> PaginationResult<T> {
    pub fn new(datalist: T, total: u64) -> Self {
        PaginationResult { datalist, total }
    }
}
