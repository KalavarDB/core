// STD Lib imports
use std::collections::HashMap;

// External crate imports
use tokio::fs::File;
use serde::{Serialize, Deserialize};

// Internal crate imports
use crate::core_structures::table_record::TableRecord;
use crate::core_structures::column::ColumnType;

/// Helper structure used to locate specific data entries for each of the tables assigned to a database
#[derive(Debug)]
pub struct DatabaseRecord {
    /// The name of the database this record represents
    pub name: String,

    /// A HashMap containing the name of each table the database can locate,
    /// as well as a way of seeing the location of all of it's data
    pub tables: HashMap<String, TableRecord>,

    /// The file that is backing this database's data
    pub backing: String,

    /// If that file is actually a directory, or just a file
    pub backing_is_dir: bool,
}

/// # The following content is undocumented due to not being ready for documentation at this time.
/// # You are welcome to attempt to make sense of it though.
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
