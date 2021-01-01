use std::env::consts::OS;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use crate::managers::{
    logging::LoggingManager,
    connections::connection_manager::ConnectionManager,
    config::ConfigManager,
};

// Main method
#[tokio::main]
async fn main() {
    println!(" LEVEL > CODE >       TIME STAMP       > MESSAGE");

    let mut logger = LoggingManager::new();
    let config_manager = ConfigManager::new(&mut logger, OS).await;
    let mut connection_manager = ConnectionManager::new(&mut logger, &config_manager, OS).await;
    connection_manager.connect(&logger).await;
}

// Sub-modules
mod managers;
mod implementors;
mod core_structures;
mod core;
mod errors;