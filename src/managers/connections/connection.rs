use std::net::SocketAddr;
use crate::managers::logging::LoggingManager;
use tokio::time::Instant;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;
use crate::managers::encryption::EncryptionManager;

pub struct Connection {
    pub id: usize,
    pub opened: Instant,
    pub remote: SocketAddr,
    pub stream: EncryptionManager,
    pub logger: LoggingManager,
    pub last_ping: Instant,
    pub receiver: Receiver<ConnectionProtocolMessage>,
    pub transmitter: Sender<ConnectionProtocolMessage>
}