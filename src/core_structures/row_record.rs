// STD Lib imports
use std::collections::HashMap;

// External crate imports
use serde::{Serialize, Deserialize};

// Internal crate imports
use crate::core_structures::table::Table;
use crate::core_structures::row::Row;

// Helper structure used to reference specific rows for a specific table in a database
#[derive(Debug)]
pub struct RowRecord {
    // A reference to the row that this record actually points to
    pub inner: Row,

    // The byte number that this row begins at in the data list
    pub start: u64,


    // The byte number that this row ends at in the data list
    pub end: u64,

    // The ID of this specific row, used for ordering
    pub id: u128,

    // The file that is backing this particular row
    pub backing: String,

    // If the "file" is actually a directory or not
    pub backing_is_dir: bool,

    // The start and end position of each of the columns within this row.
    // NOTE: this is needed for fine selection during querying of specific rows and not all rows
    pub columns: HashMap<String, (u64, u64)>
}