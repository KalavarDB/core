use tokio::net::TcpListener;
use std::collections::HashMap;
use crate::managers::storage::StorageManager;
use crate::managers::connections::connection::Connection;

pub struct ConnectionManager{
    pub listener: Option<TcpListener>,
    pub port: u32,
    pub addr: String,
    pub connections: HashMap<usize, Connection>,
    pub dbm: StorageManager
}