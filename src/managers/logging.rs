use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LoggingManager {
    pub(crate) levels: HashMap<String, bool>
}