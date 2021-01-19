use serde_derive::Deserialize;

/// A utility structure designed to parse the content of the configuration files used to make Kalavar work.
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigManager {
    /// (Platform dependant) path to the configuration file
    pub config_path: String,

    /// The actual configuration of the software
    pub config: Config,
}

/// A structure containing the root keys of the config.toml file
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Network configuration options
    pub network: NetConfig,

    /// Language related configuration options
    pub language: LanguageConfig,

    /// Logging related configuration
    pub logs: LogConfig,
}

/// Network configuration option utility structure
#[derive(Debug, Clone, Deserialize)]
pub struct NetConfig {
    /// The port which the server should attempt to bind itself to
    pub bind_port: u32,

    /// The IP address which the server should attempt to bind to
    pub bind_address: String,

    /// The maximum number of connections the server should accept and handle at any given time
    pub max_connections: u32,
}

/// Utility structure to define things relating to language used within the system
#[derive(Debug, Clone, Deserialize)]
pub struct LanguageConfig {
    /// ## Naming Conventions - Acceptable values.
    /// The following naming conventions are recognized and handled automatically by Kalvar.
    /// They are all commonly used conventions within programming languages such as: Rust, Python, Java, C#, JavaScript, etc...
    /// |Name|Variation|Identifier|
    ///
    /// |:---:|:---:|:---:|
    /// |Camel Case|Standard|CamelCase|
    /// |Camel Case|Microsoft|microsoftCamelCase|
    /// |Pascal Case|Not Applicable|PascalCase|
    /// |Snake Case|Not Applicable|snake_case|
    /// |Kebab Case|Not Applicable|kebab-case|
    /// |Screaming Case|Not Applicable|SCREAMING_CASE|
    /// |None|Not Applicable|none|
    ///
    /// If you have a suggestion for another type of naming convention you would like us to consider, please open an issue
    /// To ensure your issue gets seen by the right people, please use the `X-NAMING-CON` tag when opening your issue
    /// https://github.com/KalavarDB/core/issues


    /// The naming convention against which to validate the names of all databases, tables, columns, and procedures
    pub convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_convention: Option<bool>,

    // comment line for padding

    /// The naming convention against which to validate the names of all databases
    pub database_convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_database_convention: Option<bool>,

    // comment line for padding

    /// The naming convention against which to validate the names of all tables
    pub table_convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_table_convention: Option<bool>,

    // comment line for padding

    /// The naming convention against which to validate the names of all columns
    pub column_convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_column_convention: Option<bool>,

    // comment line for padding

    /// The naming convention against which to validate the names of all procedures
    pub procedure_convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_procedure_convention: Option<bool>,
}

/// Utility structure to define things relating to output logging used within the system
#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    /// The directory in which to place log files
    pub path: Option<String>,

    /// Enables the `DEBUG` log level
    pub debug: Option<bool>,

    /// Enables the `INFO` log level
    pub info: Option<bool>,

    /// Enables the `LOG` log level
    pub log: Option<bool>,

    /// Enables the `WARN` log level
    pub warn: Option<bool>,

    /// Enables the `ERROR` log level
    pub error: Option<bool>,
}