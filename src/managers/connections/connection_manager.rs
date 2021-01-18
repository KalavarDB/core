// External crate imports
use tokio::net::TcpListener;

// Internal crate imports
use crate::managers::storage::StorageManager;

/// A utility structure designed to simplify the management of connection handling and verification.
/// This manager also handles the launching of the memory management thread, and the storage management thread
pub struct ConnectionManager{

    /// An optional TCP Listener (optional due to being added after the struct is initiated)
    pub listener: Option<TcpListener>,

    /// The port that the listener will bind to
    pub port: u32,

    /// The IP that the listener will bind to
    pub addr: String,

    /// The id of the next connection
    pub connections: usize,

    /// The storage manager
    pub dbm: StorageManager
}
