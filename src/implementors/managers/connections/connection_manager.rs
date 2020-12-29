use crate::core_structures::connection_protocol::ConnectionProtocolMessage;
use crate::errors::ErrorMap::*;
use crate::core::utils::connection_handling::api::opcode_parser::OpCodes::*;
use crate::managers::{
    connections::connection_manager::ConnectionManager,
    config::ConfigManager,
    logging::LoggingManager,
    storage::StorageManager,
    connections::connection::Connection
};

use tokio::sync::broadcast::{Sender, Receiver, channel};
use tokio::net::TcpListener;
use tokio::time::{Instant, Duration};
use jemalloc_ctl::{epoch, stats};

impl ConnectionManager {
    pub async fn new(l: &mut LoggingManager, config: &ConfigManager, os: &str) -> ConnectionManager {
        ConnectionManager {
            listener: None,
            port: config.bind_port,
            addr: config.bind_addr.clone(),
            dbm: StorageManager::new(l, os).await,
            connections: 0,
        }
    }

    pub async fn connect(&mut self, logger: &LoggingManager) {
        let (transmitter, mut receiver): (Sender<ConnectionProtocolMessage>, Receiver<ConnectionProtocolMessage>) = channel(100);
        logger.debug_message(format!("Binding to {}:{}", &self.addr, &self.port)).await;
        let x = TcpListener::bind(format!("{}:{}", &self.addr, &self.port)).await;
        if x.is_ok() {
            // protocol handler
            let proclog = logger.clone();
            tokio::spawn(async move {
                loop {
                    let incoming = receiver.recv().await;
                    if incoming.is_ok() {
                        let payload = incoming.unwrap();
                        match payload.recipient {
                            0 => {
                                match payload.opcode {
                                    MemUse => {
                                        let used = payload.inner.2.unwrap().2;
                                        proclog.debug_message(format!("Memory usage: {:.1$}%", used, 3)).await;
                                    }
                                    _ => {
                                        proclog.debug_pretty(&payload).await;
                                    }
                                }
                            }
                            _ => {
                                //Payload is not for the protocol handler, thus should be ignored
                            }
                        }
                    } else {
                        proclog.error(incoming.unwrap_err(), EXXX).await;
                    }
                }
            });
            // memory manager
            // let memlog = logger.clone();
            let memtra: Sender<ConnectionProtocolMessage> = transmitter.clone();
            tokio::spawn(async move {
                loop {
                    epoch::advance();
                    let used = stats::allocated::read().unwrap() as f64;
                    let total = stats::resident::read().unwrap() as f64;
                    memtra.send(ConnectionProtocolMessage::new_mem(used, total));
                    tokio::time::sleep(Duration::from_secs(20)).await;
                }
            });
            let stream = x.unwrap();
            loop {
                let temp_transmitter: Sender<ConnectionProtocolMessage> = transmitter.clone();
                let temp_receiver: Receiver<ConnectionProtocolMessage> = transmitter.subscribe();
                self.connections += 1;
                let connection = Connection::new(self.connections, &logger, stream.accept().await.unwrap(), temp_receiver, temp_transmitter).await;
                tokio::spawn(async move {
                    connection.transmitter.send(ConnectionProtocolMessage::new_con(&connection.id));
                    crate::core::utils::connection_handling::handle(connection)
                });
            }
        } else {
            logger.fatal(x.unwrap_err(), EXXX, 1).await;
        }
    }
}