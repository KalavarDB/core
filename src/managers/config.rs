use serde_derive::Deserialize;

/// A utility structure designed to parse the content of the configuration files used to make Kalavar work.
#[derive(Debug, Clone)]
pub struct ConfigManager {
    /// (Platform dependant) path to the configuration file
    pub config_path: String,

    /// The actual configuration of the software
    pub config: Config,
}

/// A structure containing the root keys of the config.toml file
#[derive(Debug, Clone, Deserialize)]
pub struct PreConfig {
    /// Network configuration options
    pub network: Option<PreNetConfig>,

    /// Language related configuration options
    pub language: Option<PreLanguageConfig>,

    /// Logging related configuration
    pub logs: Option<PreLogConfig>,
}

/// Network configuration option utility structure
#[derive(Debug, Clone, Deserialize)]
pub struct PreNetConfig {
    /// The port which the server should attempt to bind itself to
    #[serde(rename = "bind-port")]
    pub bind_port: Option<u32>,

    /// The IP address which the server should attempt to bind to
    #[serde(rename = "bind-address")]
    pub bind_address: Option<String>,

    /// The maximum number of connections the server should accept and handle at any given time
    #[serde(rename = "max-connections")]
    pub max_connections: Option<u32>,
}


/// A utility structure to define things relating to language used within the system
///
/// ## Naming Conventions - Acceptable values.
/// The following naming conventions are recognized and handled automatically by Kalvar.<br>
/// They are all commonly used conventions within programming languages such as: Rust, Python, Java, C#, JavaScript, etc...
///
/// |Name|Variation|Identifier|
/// |:---:|:---:|:---:|
/// |Camel Case|Standard|CamelCase|
/// |Camel Case|Microsoft|microsoftCamelCase|
/// |Pascal Case|Not Applicable|PascalCase|
/// |Snake Case|Not Applicable|snake_case|
/// |Kebab Case|Not Applicable|kebab-case|
/// |Screaming Case|Not Applicable|SCREAMING_CASE|
/// |None|Not Applicable|none|
///
/// If you have a suggestion for another type of naming convention you would like us to consider, please open an issue <br>
/// To ensure your issue gets seen by the right people, please use the `X-NAMING-CON` tag when opening your issue <br>
/// [Open Issue](https://github.com/KalavarDB/core/issues)
#[derive(Debug, Clone, Deserialize)]
pub struct PreLanguageConfig {
    /// The naming convention against which to validate the names of all databases, tables, columns, and procedures
    pub convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_convention: Option<bool>,

    // comment line for padding

    /// The naming convention against which to validate the names of all databases
    #[serde(rename = "database-convention")]
    pub database_convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    #[serde(rename = "force-database-convention")]
    pub force_database_convention: Option<bool>,

    // comment line for padding

    /// The naming convention against which to validate the names of all tables
    #[serde(rename = "table-convention")]
    pub table_convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    #[serde(rename = "force-table-convention")]
    pub force_table_convention: Option<bool>,

    // comment line for padding

    /// The naming convention against which to validate the names of all columns
    #[serde(rename = "column-convention")]
    pub column_convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    #[serde(rename = "force-column-convention")]
    pub force_column_convention: Option<bool>,

    // comment line for padding

    /// The naming convention against which to validate the names of all procedures
    #[serde(rename = "procedure-convention")]
    pub procedure_convention: Option<String>,

    /// Whether or not to force the naming convention, or to simply provide a warning
    #[serde(rename = "force-procedure-convention")]
    pub force_procedure_convention: Option<bool>,
}

/// Utility structure to define things relating to output logging used within the system
#[derive(Debug, Clone, Deserialize)]
pub struct PreLogConfig {
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

/// A structure containing the root keys of the config.toml file
#[derive(Debug, Clone)]
pub struct Config {
    /// Network configuration options
    pub network: NetConfig,

    /// Language related configuration options
    pub language: LanguageConfig,

    /// Logging related configuration
    pub logs: LogConfig,
}

/// Network configuration option utility structure
#[derive(Debug, Clone)]
pub struct NetConfig {
    /// The port which the server should attempt to bind itself to
    pub bind_port: u32,

    /// The IP address which the server should attempt to bind to
    pub bind_address: String,

    /// The maximum number of connections the server should accept and handle at any given time
    pub max_connections: u32,
}


/// A utility structure to define things relating to language used within the system
///
/// ## Naming Conventions - Acceptable values.
/// The following naming conventions are recognized and handled automatically by Kalvar.<br>
/// They are all commonly used conventions within programming languages such as: Rust, Python, Java, C#, JavaScript, etc...
///
/// |Name|Variation|Identifier|
/// |:---:|:---:|:---:|
/// |Camel Case|Standard|CamelCase|
/// |Camel Case|Microsoft|microsoftCamelCase|
/// |Pascal Case|Not Applicable|PascalCase|
/// |Snake Case|Not Applicable|snake_case|
/// |Kebab Case|Not Applicable|kebab-case|
/// |Screaming Case|Not Applicable|SCREAMING_CASE|
/// |None|Not Applicable|none|
///
/// If you have a suggestion for another type of naming convention you would like us to consider, please open an issue <br>
/// To ensure your issue gets seen by the right people, please use the `X-NAMING-CON` tag when opening your issue <br>
/// [Open Issue](https://github.com/KalavarDB/core/issues)
#[derive(Debug, Clone)]
pub struct LanguageConfig {
    /// The naming convention against which to validate the names of all databases, tables, columns, and procedures
    pub convention: String,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_convention: bool,

    // comment line for padding

    /// The naming convention against which to validate the names of all databases
    pub database_convention: String,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_database_convention: bool,

    // comment line for padding

    /// The naming convention against which to validate the names of all tables
    pub table_convention: String,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_table_convention: bool,

    // comment line for padding

    /// The naming convention against which to validate the names of all columns
    pub column_convention: String,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_column_convention: bool,

    // comment line for padding

    /// The naming convention against which to validate the names of all procedures
    pub procedure_convention: String,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_procedure_convention: bool,
}

/// Utility structure to define things relating to output logging used within the system
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// The directory in which to place log files
    pub path: String,

    /// Enables the `DEBUG` log level
    pub debug: bool,

    /// Enables the `INFO` log level
    pub info: bool,

    /// Enables the `LOG` log level
    pub log: bool,

    /// Enables the `WARN` log level
    pub warn: bool,

    /// Enables the `ERROR` log level
    pub error: bool,
}


impl PreConfig {
    pub fn convert(self) -> Config {
        let net = self.network.unwrap_or(PreNetConfig {
            bind_port: Some(1234),
            bind_address: Some("localhost".to_string()),
            max_connections: Some(25),
        });

        let lang = self.language.unwrap_or(PreLanguageConfig {
            convention: Some("none".to_string()),
            force_convention: Some(true),
            database_convention: Some("none".to_string()),
            force_database_convention: Some(true),
            table_convention: Some("none".to_string()),
            force_table_convention: Some(true),
            column_convention: Some("none".to_string()),
            force_column_convention: Some(true),
            procedure_convention: Some("none".to_string()),
            force_procedure_convention: Some(true),
        });

        let log = self.logs.unwrap_or(PreLogConfig {
            path: Some("/var/lib/kalavar/logs".to_string()),
            debug: Some(true),
            info: Some(true),
            log: Some(true),
            warn: Some(true),
            error: Some(true),
        });

        Config {
            network: NetConfig {
                bind_port: net.bind_port.unwrap_or(1234),
                bind_address: net.bind_address.unwrap_or("localhost".to_string()),
                max_connections: net.max_connections.unwrap_or(25),
            },
            language: LanguageConfig {
                convention: lang.convention.unwrap_or("none".to_string()),
                force_convention: lang.force_convention.unwrap_or(false),
                database_convention: lang.database_convention.unwrap_or("none".to_string()),
                force_database_convention: lang.force_database_convention.unwrap_or(false),
                table_convention: lang.table_convention.unwrap_or("none".to_string()),
                force_table_convention: lang.force_table_convention.unwrap_or(false),
                column_convention: lang.column_convention.unwrap_or("none".to_string()),
                force_column_convention: lang.force_column_convention.unwrap_or(false),
                procedure_convention: lang.procedure_convention.unwrap_or("none".to_string()),
                force_procedure_convention: lang.force_procedure_convention.unwrap_or(false),
            },
            logs: LogConfig {
                path: log.path.unwrap_or("/var/lib/kalavar/logs".to_string()),
                debug: log.debug.unwrap_or(true),
                info: log.info.unwrap_or(true),
                log: log.log.unwrap_or(true),
                warn: log.warn.unwrap_or(true),
                error: log.error.unwrap_or(true),
            },
        }
    }
}