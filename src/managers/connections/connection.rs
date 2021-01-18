use std::net::SocketAddr;

use tokio::time::Instant;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::managers::logging::LoggingManager;
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;
use crate::managers::encryption::EncryptionManager;


/// A structure used to represent a connection to the system from an external client
pub struct Connection {
    /// The identification number of the connection, used by the manager to broadcast messages
    pub id: usize,

    /// The time at which the connection was initialized
    pub opened: Instant,

    /// The address of the connection
    pub remote: SocketAddr,

    /// The datastream of the client, consists of the encryption manager among other things
    pub stream: EncryptionManager,

    /// The distinctive logging manager for this client,
    pub logger: LoggingManager,

    /// The timestamp of the last time the connection was validated to not be zombied
    pub last_ping: Instant,

    /// The receiver for messages from the connection manager to the client
    pub receiver: Receiver<ConnectionProtocolMessage>,

    /// The transmitter used to transmit messsages to the connection manager from the client
    pub transmitter: Sender<ConnectionProtocolMessage>,
}