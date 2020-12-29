#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;


use std::env::consts::OS;

use jemalloc_ctl::{epoch, stats};

use tokio::net::TcpListener;
use tokio::sync::broadcast::{channel, Sender, Receiver};

use crate::managers::{
    logging::LoggingManager,
    connections::connection_manager::ConnectionManager,
    config::ConfigManager,
};
use crate::core_structures::as_bytes::AsBytes;

mod managers;
mod implementors;
mod core_structures;
mod core;
mod errors;

#[tokio::main]
async fn main() {
    println!(" LEVEL > CODE >       TIME STAMP       > MESSAGE");
    let mut logger = LoggingManager::new();

    let config_manager = ConfigManager::new(&mut logger, OS).await;
    let mut connection_manager = ConnectionManager::new(&mut logger, &config_manager, OS).await;
    let test_string = "Hello, World!".to_string();
    logger.debug_message(&test_string).await;
    logger.debug(&test_string.as_kv_bytes()).await;
    connection_manager.connect(&logger).await;
}