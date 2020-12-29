use std::collections::HashMap;
use crate::core_structures::column::ColumnType;
use crate::core_structures::row::Row;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: HashMap<String, ColumnType>,
    pub rows: Vec<Row>
}

impl Table {
    pub fn new(name: String, columns: Vec<(String, ColumnType)>) -> Table {
        let mut t = Table {
            name,
            columns: HashMap::new(),
            rows: vec!()
        };

        for column in columns {
            t.columns.insert(column.0, column.1);
        }

        t
    }
}