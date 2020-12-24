use crate::managers::connections::connection_manager::ConnectionManager;
use crate::managers::config::ConfigManager;
use std::collections::HashMap;
use crate::managers::logging::LoggingManager;
use crate::managers::storage::StorageManager;
use tokio::sync::broadcast::{Sender, Receiver, channel};
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;
use crate::managers::connections::connection::Connection;
use tokio::net::TcpListener;

impl ConnectionManager {
    pub async fn new(l: &LoggingManager, config: &ConfigManager, os: &str) -> ConnectionManager {
        ConnectionManager {
            listener: None,
            port: config.bind_port,
            addr: config.bind_addr.clone(),
            connections: HashMap::new(),
            dbm: StorageManager::new(l, os).await,
        }
    }

    pub async fn connect(&mut self, logger: &LoggingManager) {
        let (transmitter, mut receiver): (Sender<ConnectionProtocolMessage>, Receiver<ConnectionProtocolMessage>) = channel(100);
        logger.debug_message(format!("Binding to {}:{}", &self.addr, &self.port));
        let x = TcpListener::bind(format!("{}:{}", &self.addr, &self.port)).await;
        if x.is_ok() {
            let log = logger.clone();
            tokio::spawn(async move {
                loop {
                    let incoming = receiver.recv().await;
                    if incoming.is_ok() {
                        log.debug(incoming.unwrap());
                    } else {
                        log.error(incoming.unwrap_err());
                    }
                }
            });
            let stream = x.unwrap();
            loop {
                let temp_transmitter: Sender<ConnectionProtocolMessage> = transmitter.clone();
                let temp_receiver: Receiver<ConnectionProtocolMessage> = transmitter.subscribe();
                let connection = Connection::new(&logger, stream.accept().await.unwrap(), temp_receiver, temp_transmitter);
                tokio::spawn(async move {
                    crate::core::utils::connection_handling::handle(connection)
                });
            }
        } else {
            logger.fatal(x.unwrap_err(), 1);
        }
    }
}