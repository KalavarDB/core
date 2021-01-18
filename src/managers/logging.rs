use std::collections::HashMap;


/// A utility structure designed to assist in the processing of log messages across the entire system.
#[derive(Debug, Clone)]
pub struct LoggingManager {
    /// Defines the different log levels, auto populated at generation time
    pub(crate) levels: HashMap<String, bool>,

    /// Defines the relative path to the log file
    pub log_file: String,

    /// Defines the distinctive path to the log file
    pub log_file_full: String
}