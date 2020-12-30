use std::net::SocketAddr;
use tokio::net::TcpStream;
use crate::managers::logging::LoggingManager;
use tokio::time::Instant;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;

pub struct Connection {
    pub id: usize,
    pub opened: Instant,
    pub remote: SocketAddr,
    pub stream: TcpStream,
    pub logger: LoggingManager,
    pub last_ping: Instant,
    pub receiver: Receiver<ConnectionProtocolMessage>,
    pub transmitter: Sender<ConnectionProtocolMessage>
}