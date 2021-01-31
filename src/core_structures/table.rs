// STD Lib imports
use std::collections::HashMap;

// External crate imports
use serde_derive::{Serialize, Deserialize};

// Internal crate imports
use crate::core_structures::column::{ColumnType, ColumnTypeEnum};

/// A structure defining the contents of a table within a database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    /// The name of the table this struct references
    pub name: String,

    /// The columns (and their types) of this table
    pub columns: HashMap<String, ColumnType>,
}
#[allow(dead_code)]

impl Table {
    /// Helper method designed to instantiate a new table on behalf of the caller
    pub fn new<N: Into<String>>(name: N, columns: Vec<(N, ColumnType)>) -> Table {
        let mut t = Table {
            name: name.into(),
            columns: HashMap::new(),
        };

        t.columns.insert("__IDENTIFIER__".to_string(), ColumnType::new_prv(ColumnTypeEnum::Integer64, None, 64));

        for column in columns {
            t.columns.insert(column.0.into(), column.1);
        }

        t
    }
}