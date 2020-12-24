use std::collections::HashMap;
use tokio::fs::File;

#[derive(Debug, Clone)]
pub struct DatabaseRecord {
    pub name: String,
    pub tables: HashMap<String, (u32, u64)>,
    pub backing: File,
}

impl DatabaseRecord {
    pub fn new(name: String, backing: File) -> DatabaseRecord {
        DatabaseRecord {
            name,
            tables: Default::default(),
            backing
        }
    }
}