use std::env::consts::OS;

use tokio::net::TcpListener;
use tokio::sync::broadcast::{channel, Sender, Receiver};
use crate::managers::{
    logging::LoggingManager,
    connections::connection_manager::ConnectionManager,
    config::ConfigManager,
};

mod managers;
mod implementors;
mod core_structures;
mod core;


#[tokio::main]
async fn main() {
    let mut logger = LoggingManager::new();
    let config_manager = ConfigManager::new(&mut logger, OS).await;
    let mut connection_manager = ConnectionManager::new(&logger, &config_manager, OS).await;
    connection_manager.connect(&logger).await;
}