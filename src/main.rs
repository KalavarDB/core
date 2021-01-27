#![doc(html_logo_url = "https://kalavar.cf/assets/images/k_transparent.png", html_favicon_url = "https://kalavar.cf/favicon.ico", issue_tracker_base_url = "https://github.com/KalavarDB/core/issues/", html_root_url = "https://dev.kalavar.cf/", html_no_source)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

/// # The code you are currently reading is highly early development, and may contain things which are useless at this time.
/// # That also means that this code should not be taken as final, and any content within is subject to change without notice.
/// # For further information, I recommend looking at the documentation at the link below.
/// # Docs: https://kalavar.cf/documentation
// Used by many structures and methods within the program to determine where files are located
use std::env::consts::OS;

use crate::managers::{
    logging::LoggingManager,
    connections::connection_manager::ConnectionManager,
    config::post::ConfigManager,
};
use crate::managers::analytics::AnalyticsManager;

// The following basically just sets the global allocator to use the Jemalloc allocator so we can track memory usage.
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;


// The primary function of the program, called at runtime to start the server
#[tokio::main]
async fn main() {
    // Instantiate a new instance of the logging manager
    // This is important as the program requires this at all levels for debugging
    let mut logger = LoggingManager::new();

    logger.init().await;

    // Instantiate a new instance of the config manager
    // Used to parse the configuration file into something the server can make use of
    let config_manager = ConfigManager::new(&mut logger, OS).await;

    // Instantiate a new connection manager
    // This structure manages all of the core components of the database
    // Ranging from the connections themselves, to the memory manager, and even the storage manager
    // Without the following two lines of code, the program would simply exit and do nothing.
    let mut connection_manager = ConnectionManager::new(&mut logger, &config_manager, OS).await;
    connection_manager.connect(&logger, AnalyticsManager::new(config_manager.config.privacy.mode.clone())).await;
}

// Sub-modules
/// Structure definitions for all of the management structures used within the program
mod managers;
/// Structure definitions for things like databases and tables
mod core_structures;
/// A selection of utilities and helper methods to ensure that the program runs smoothyl
mod core;
/// Custom defined error types used by the program to report clear, concise error messages back to the user so they can get their database up and running ASAP
mod errors;

/// A module containing all the information regarding compilers used in the project.
mod compilers;

#[cfg(test)]
mod tests;
