use crate::core_structures::database_record::DatabaseRecord;
use std::collections::HashMap;
use crate::core_structures::table::Table;
use tokio::fs::File;

#[derive(Debug, Clone)]
pub struct Database {
    pub backing: File,
    pub name: String,
    pub tables: HashMap<String, Table>,
    pub record: DatabaseRecord
    // pub permissions: HashMap<User, Permissions>
}

impl Database {
    pub fn new<A: Into<String>>(name: A, self_backing: File, record_backing: File, tables: Vec<(String, Table)>) -> Database {
        let n = name.into();
        Database {
            backing: self_backing,
            name: n.clone(),
            tables: HashMap::new(),
            record: DatabaseRecord::new(n, record_backing)
        }
    }
}