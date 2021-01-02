use crate::core_structures::table::Table;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core_structures::row::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct RowRecord {
    pub inner: Row,
    pub start: u64,
    pub end: u64,
    pub id: u128,
    pub backing: String,
    pub backing_is_dir: bool,
    pub columns: HashMap<String, (u64, u64)>
}