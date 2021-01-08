// STD lib imports
use std::collections::HashMap;
use std::time::Instant;

// Internal crate imports
use crate::core_structures::database_record::DatabaseRecord;

// A structure built to manage the basic functions of storage and retrieval of data
// Literally the most important structure of the whole database, give it some credit
#[derive(Debug)]
pub struct StorageManager {
    // A HashMap containing the name of each database the manager can locate,
    // as well as a way of seeing the location of all of it's data
    pub databases: HashMap<String, DatabaseRecord>,

    // The last time the database was dumped to the disk
    pub last_write: Instant,

    // The directory in which it will find any database it should read and process
    pub dir: String,
}
