use crate::core_structures::database_record::DatabaseRecord;
use std::collections::HashMap;
use crate::core_structures::table::Table;
use tokio::fs::File;

#[derive(Debug)]
pub struct Database {
    pub backing: File,
    pub name: String,
    pub tables: HashMap<String, Table>,
    // pub permissions: HashMap<User, Permissions>
}

impl Database {
    pub fn new<A: Into<String>>(name: A, self_backing: File) -> Database {
        let n = name.into();
        Database {
            backing: self_backing,
            name: n.clone(),
            tables: HashMap::new(),
        }
    }
}