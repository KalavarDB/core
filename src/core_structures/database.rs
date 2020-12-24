use crate::core_structures::database_record::DatabaseRecord;
use std::collections::HashMap;
use crate::core_structures::table::Table;

#[derive(Debug, Clone)]
pub struct Database {
    pub backing: String,
    pub tables: HashMap<String, Table>,
    pub record: DatabaseRecord
    // pub permissions: HashMap<User, Permissions>
}