use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LoggingManager {
    pub(crate) levels: HashMap<String, bool>,
    pub log_file: String,
    pub log_file_full: String
}