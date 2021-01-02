use std::collections::HashMap;
use tokio::fs::File;

use crate::core_structures::table_record::TableRecord;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
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
            tables: Default::default(),
            backing,
            backing_is_dir: false
        }
    }
}