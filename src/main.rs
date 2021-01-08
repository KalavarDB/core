/// # The code you are currently reading is highly early development, and may contain things which are useless at this time.
/// # That also means that this code should not be taken as final, and any content within is subject to change without notice.
/// # For further information, I recommend looking at the documentation at the link below.
/// # Docs: https://kalavar.cf/documentation

// Used by many structures and methods within the program to determine where files are located
use std::env::consts::OS;

// The following basically just sets the global allocator to use the Jemalloc allocator so we can track memory usage.
// If a target is not supported by jemalloc we cant compile for it
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;


use crate::managers::{
    logging::LoggingManager,
    connections::connection_manager::ConnectionManager,
    config::ConfigManager,
};

// The primary function of the program, called at runtime to start the server
#[tokio::main]
async fn main() {
    // Print the headers for the log table to the system
    println!(" LEVEL > CODE >       TIME STAMP       > MESSAGE");

    // Instantiate a new instance of the logging manager
    // This is important as the program requires this at all levels for debugging
    let mut logger = LoggingManager::new();

    // Instantiate a new instance of the config manager
    // Used to parse the configuration file into something the server can make use of
    let config_manager = ConfigManager::new(&mut logger, OS).await;

    // Instantiate a new connection manager
    // This structure manages all of the core components of the database
    // Ranging from the connections themselves, to the memory manager, and even the storage manager
    // Without the following two lines of code, the program would simply exit and do nothing.
    let mut connection_manager = ConnectionManager::new(&mut logger, &config_manager, OS).await;
    connection_manager.connect(&logger).await;
}

// Sub-modules
mod managers;
mod implementors;
mod core_structures;
mod core;
mod errors;