use crate::core_structures::table::Table;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core_structures::row_record::RowRecord;
use crate::core_structures::column::ColumnType;

#[derive(Debug)]
pub struct TableRecord {
    pub inner: Table,
    pub start: u64,
    pub end: u64,
    pub backing: String,
    pub backing_is_dir: bool,
    pub name: String,
    pub columns: HashMap<String, ColumnType>,
    pub rows: HashMap<u128, RowRecord>
}

impl TableRecord {
    pub fn new(name: &str, columns: &Vec<(String, ColumnType)>, start: u64) -> TableRecord {
        let mut t = TableRecord {
            inner: Table {
                name: name.to_string(),
                columns: HashMap::new(),
                rows: vec![]
            },
            start,
            end: start+1,
            backing: "".to_string(),
            backing_is_dir: false,
            name: name.to_string(),
            columns: HashMap::new(),
            rows: HashMap::new()
        };

        for column in columns {

            t.inner.columns.insert(column.0.clone(), column.1.clone());
            t.columns.insert(column.0.clone(), column.1.clone());
        }

        t
    }
}