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

    /// Language related configuration options
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

/// Utility structure to define things relating to language used within the system
pub struct LanguageConfig {
    /// The naming convention against which to validate the names of all databases, tables, columns, and procedures
    pub convention: Option<String>,

    /// The naming convention against which to validate the names of all databases
    pub database_convention: Option<String>,

    /// The naming convention against which to validate the names of all tables
    pub table_convention: Option<String>,

    /// The naming convention against which to validate the names of all columns
    pub column_convention: Option<String>,

    /// The naming convention against which to validate the names of all procedures
    pub procedure_convention: Option<String>,
}