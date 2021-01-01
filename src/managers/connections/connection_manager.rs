use tokio::net::TcpListener;
use crate::managers::storage::StorageManager;

pub struct ConnectionManager{
    pub listener: Option<TcpListener>,
    pub port: u32,
    pub addr: String,
    pub connections: usize,
    pub dbm: StorageManager
}
