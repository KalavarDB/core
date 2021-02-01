// STD lib imports
use std::collections::HashMap;
use std::time::Instant;

// Internal crate imports
use crate::core_structures::database_record::DatabaseRecord;
use tokio::fs::File;


/// Utility structure whose only job is to manage the smooth transfer of data between memory and disk based on cache capacity, memory usage, and a number of other factors
#[derive(Debug)]
pub struct StorageManager {
    /// A tracking map of all the databases that the program has located within its data directory
    pub databases: HashMap<String, DatabaseRecord>,

    /// The last time that any in-memory data was dumped to the disk and refreshed
    pub last_write: Instant,

    /// The directory to scan for potential databases
    pub dir: String,

    /// The list of DatabaseReferences from which to pull data
    pub map: HashMap<String, DatabaseReference>
}

/// Helper structure for storing file references within the storeage manager for reading and writing at a later time
#[derive(Debug)]
pub struct DatabaseReference {
    pub core: File
}