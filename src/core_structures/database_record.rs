use std::collections::HashMap;
use tokio::fs::File;

use crate::core_structures::table_record::TableRecord;
use serde::{Serialize, Deserialize};
use crate::core_structures::column::ColumnType;

#[derive(Debug)]
pub struct DatabaseRecord {
    pub name: String,
    pub tables: HashMap<String, TableRecord>,
    pub backing: String,
    pub backing_is_dir: bool,
}

impl DatabaseRecord {
    pub fn new(name: String, backing: String) -> DatabaseRecord {
        DatabaseRecord {
            name,
            tables: HashMap::new(),
            backing,
            backing_is_dir: false
        }
    }

    pub fn new_table(&mut self, name: String, columns: Vec<(String, ColumnType)>) {
        let mut start: u64 = 0;

        if self.tables.len() > 0 {
            let last = self.tables.iter().last().unwrap().1;
            start = last.end+1;
        }
        let table = TableRecord::new(name.as_str(), &columns, start);

        self.tables.insert(name, table);
    }

    pub fn read_table(&self, name: &str) {
        if self.tables.contains_key(name) {

        }
    }

}
