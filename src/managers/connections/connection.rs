use std::net::SocketAddr;
use tokio::net::TcpStream;
use crate::managers::logging::LoggingManager;
use tokio::time::Instant;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;

pub struct Connection {
    pub id: usize,
    pub remote: SocketAddr,
    pub stream: TcpStream,
    pub logger: LoggingManager,
    pub last_ping: Instant,
    pub receiver: Receiver<ConnectionProtocolMessage>,
    pub transmitter: Sender<ConnectionProtocolMessage>
}

impl Connection {
    pub async fn read_incoming(&mut self) /*-> Result<ConnectionProtocolMessage>*/ {

    }
}