// External crate imports
use tokio::net::TcpListener;

// Internal crate imports
use crate::managers::storage::StorageManager;

// Management structure to handle the connection server, and instnatiate the storage manager, w
// hilst launching threads for managing memory allocation and queries
pub struct ConnectionManager{

    // An optional TCP Listener (optional due to being added after the struct is initiated)
    pub listener: Option<TcpListener>,

    // The port that the listener will bind to
    pub port: u32,

    // The IP that the listener will bind to
    pub addr: String,

    // The id of the next connection
    pub connections: usize,

    // The storage manager
    pub dbm: StorageManager
}
