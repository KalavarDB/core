/// A utility structure designed to parse the content of the configuration files used to make Kalavar work.
#[derive(Debug, Clone)]
pub struct ConfigManager {
    /// (Platform dependant) path to the configuration file
    pub config_path: String,

    /// The actual configuration of the software
    pub config: Config,
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

    /// Privacy configuration settings
    pub privacy: PrivacyConfig,
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

    /// The IP ranges from which to accept connections
    /// Accepts CIDR and range notation (see public documentation)
    pub accept_ranges: Vec<String>,
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
    pub convention: NamingConvention,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_convention: bool,

    // comment line for padding

    /// The naming convention against which to validate the names of all databases
    pub database_convention: NamingConvention,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_database_convention: bool,

    // comment line for padding

    /// The naming convention against which to validate the names of all tables
    pub table_convention: NamingConvention,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_table_convention: bool,

    // comment line for padding

    /// The naming convention against which to validate the names of all columns
    pub column_convention: NamingConvention,

    /// Whether or not to force the naming convention, or to simply provide a warning
    pub force_column_convention: bool,

    // comment line for padding

    /// The naming convention against which to validate the names of all procedures
    pub procedure_convention: NamingConvention,

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

    /// Enables the logging manager to actually create and write log files to disk
    pub log_to_file: bool,
}

/// A utility structure for defining things related to the privacy of the user
#[derive(Debug, Clone)]
pub struct PrivacyConfig {
    /// The mode under which data will be shared
    pub mode: PrivacyMode,
}


/// An enumerator for making privacy checking simpler
#[derive(Debug, Clone)]
pub enum PrivacyMode {
    /// no data sharing, see [here](https://kalavar.cf/documentation/configuration/privacy/)
    None,

    /// sharing of minimal information, see [here](https://kalavar.cf/documentation/configuration/privacy/)
    Minimal,

    /// sharing of basic information, see [here](https://kalavar.cf/documentation/configuration/privacy/)
    Basic,

    /// sharing of detailed information, see [here](https://kalavar.cf/documentation/configuration/privacy/)
    Detailed,
}


/// An enumerator for making naming convention simpler
#[derive(Debug, Clone)]
pub enum NamingConvention {
    /// Value defining UpperCamelCase
    UpperCamelCase,

    /// Value defining lowerCamelCase
    LowerCamelCase,

    /// Value defining PascalCase
    PascalCase,

    /// Value defining UPPER_SNAKE_CASE
    UpperSnakeCase,

    /// Value defining lower_snake_case
    LowerSnakeCase,

    /// Value defining kebab-case
    KebabCase,

    /// Value defining no naming convention
    None,
}