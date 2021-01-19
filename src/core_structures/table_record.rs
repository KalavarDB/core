// STD Lib imports
use std::collections::HashMap;

// External crate imports

// Internal crate imports
use crate::core_structures::table::Table;
use crate::core_structures::row_record::RowRecord;
use crate::core_structures::column::ColumnType;

/// Helper structure used to locate specific rows for a specific table in a database
#[derive(Debug)]
pub struct TableRecord {
    /// A reference to the table that this record actually points to
    pub inner: Table,

    /// The byte number that this table begins at in the data list
    pub start: u64,

    /// The byte signifying the end of this table
    pub end: u64,

    /// The name of the file backing this table
    pub backing: String,

    /// If the backing is actually a directory, or just a file
    pub backing_is_dir: bool,

    /// The name of the table this struct is referencing
    pub name: String,

    /// The columns of the table, and their respective types
    pub columns: HashMap<String, ColumnType>,

    /// The rows within the table itself
    pub rows: HashMap<u128, RowRecord>
}

/// # The following content is undocumented due to not being ready for documentation at this time.
/// # You are welcome to attempt to make sense of it though.
impl TableRecord {
    #[allow(dead_code)]
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