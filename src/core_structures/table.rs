// STD Lib imports
use std::collections::HashMap;

// Internal crate imports
use crate::core_structures::column::{ColumnType, ColumnTypeEnum};
use crate::core_structures::row::Row;

// A structure defining the contents of a table within a database
#[derive(Debug, Clone)]
pub struct Table {
    // The name of the table this struct references
    pub name: String,

    // The columns (and their types) of this table
    pub columns: HashMap<String, ColumnType>,

    // An (unsorted) array of all the rows in this table
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new<N: Into<String>>(name: N, columns: Vec<(N, ColumnType)>) -> Table {
        let mut t = Table {
            name: name.into(),
            columns: HashMap::new(),
            rows: vec!(),
        };
        t.columns.insert("__IDENTIFIER__".to_string(), ColumnType::new_prv(ColumnTypeEnum::Integer64, None, 64));
        for column in columns {
            t.columns.insert(column.0.into(), column.1);
        }

        t
    }
}