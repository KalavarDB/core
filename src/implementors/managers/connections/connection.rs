use crate::managers::connections::connection::Connection;
use tokio::net::TcpStream;
use crate::managers::logging::LoggingManager;
use std::net::SocketAddr;
use tokio::time::Instant;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;
use crate::managers::encryption::EncryptionManager;

impl Connection {
    pub async fn new(id: usize, logger: &LoggingManager, (stream, remote): (TcpStream, SocketAddr), receiver: Receiver<ConnectionProtocolMessage>, transmitter: Sender<ConnectionProtocolMessage>) -> Connection {
        logger.debug_message(format!("Incoming connection from {}", remote.ip())).await;
        Connection {
            id,
            opened: Instant::now(),
            remote,
            stream: EncryptionManager::new(stream),
            logger: logger.clone(),
            last_ping: Instant::now(),
            receiver,
            transmitter,
        }
    }
}