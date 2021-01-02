use crate::core_structures::table::Table;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core_structures::row_record::RowRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct TableRecord {
    pub inner: Table,
    pub start: u64,
    pub end: u64,
    pub backing: String,
    pub backing_is_dir: bool,
    pub name: String,
    pub rows: HashMap<u128, RowRecord>
}