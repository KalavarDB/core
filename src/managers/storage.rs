use std::collections::HashMap;
use std::time::Instant;
use crate::core_structures::database::Database;

pub struct StorageManager {
    pub databases: HashMap<String, Database>,
    pub last_write: Instant
}