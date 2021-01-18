use serde::Serialize;

/// A utility structure designed to parse the content of the configuration files used to make Kalavar work.
#[derive(Debug, Clone)]
pub struct ConfigManager {
    /// (Platform dependant) path to the configuration file
    pub config_path: String,

    /// The actual configuration of the software
    pub config: Config,
}

/// A structure containing the root keys of the config.toml file
pub struct Config {
    /// Network configuration options
    pub network: NetConfig,
    pub processing: ProcessingConfig,
    pub language: LanguageConfig,
}

/// Network configuration option utility structure
pub struct NetConfig {
    /// The port which the server should attempt to bind itself to
    pub bind_port: u32,

    /// The IP address which the server should attempt to bind to
    pub bind_addr: String,

    /// The maximum number of connections the server should accept and handle at any given time
    pub max_connections: u32,

    /// The maximum number of connections that the server should run on each thread
    pub connections_per_thread: u8,
}

pub struct ProcessingConfig {
    // The maximum number of threads the server should run at a time
    pub max_threads: u8,
}

pub struct LanguageConfig {
    // The
}