use std::collections::HashMap;
use std::time::Instant;
use crate::core_structures::database_record::DatabaseRecord;

#[derive(Debug)]
pub struct StorageManager {
    pub databases: HashMap<String, DatabaseRecord>,
    pub last_write: Instant,
    pub dir: String,
}
