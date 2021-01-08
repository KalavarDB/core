use std::collections::HashMap;


// A structure designed for managing the debugging of the program.
// It will automatically write to a file and the terminal if it has permission to modify and create files
#[derive(Debug, Clone)]
pub struct LoggingManager {
    // Used to define the differnt log levels, auto populated at generation time
    pub(crate) levels: HashMap<String, bool>,

    // Used to show the relative path to the log file
    pub log_file: String,

    // Used to show the distinctive path to the log file
    pub log_file_full: String
}