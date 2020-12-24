#[derive(Debug, Clone)]
pub struct ConfigManager {
    pub config_path: String,
    pub bind_port: u32,
    pub bind_addr: String,
    pub max_connections: u32,
    pub connections_per_thread: u8,
    pub max_threads: u8,
}