use serde_derive::Deserialize;
use crate::managers::config::post::{Config, NetConfig, PrivacyConfig, PrivacyMode, LogConfig, LanguageConfig, NamingConvention};

/// A structure containing the root keys of the config.toml file
#[derive(Debug, Clone, Deserialize)]
pub struct PreConfig {
    /// Network configuration options
    pub network: Option<PreNetConfig>,

    /// Language related configuration options
    pub language: Option<PreLanguageConfig>,

    /// Logging related configuration
    pub logs: Option<PreLogConfig>,

    /// Privacy configuration settings
    pub privacy: Option<PrePrivacyConfig>,
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

    /// The IP address ranges from which to accept incoming connections
    #[serde(rename = "accept-ranges")]
    pub accept_ranges: Option<Vec<String>>,
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

    /// Enables the logging manager to create and write log files whilst the program is running
    #[serde(rename = "log-to-file")]
    pub log_to_file: Option<bool>,
}

/// Utility structure to define the configuration of the privacy module
#[derive(Debug, Clone, Deserialize)]
pub struct PrePrivacyConfig {
    pub mode: Option<String>
}

impl PreConfig {
    pub fn convert(self) -> Config {
        let net = self.network.unwrap_or(PreNetConfig {
            bind_port: Some(1234),
            bind_address: Some("localhost".to_string()),
            max_connections: Some(25),
            accept_ranges: Some(vec!["*".to_string()]),
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
            log_to_file: Some(true),
        });

        let pre_privacy = self.privacy.unwrap_or(PrePrivacyConfig {
            mode: Some("none".to_string()),
        });

        let privacy = PrivacyConfig {
            mode: match pre_privacy.mode.unwrap().to_lowercase().as_str() {
                "detailed" => PrivacyMode::Detailed,
                "basic" => PrivacyMode::Basic,
                "minimal" => PrivacyMode::Minimal,
                _ => PrivacyMode::None
            }
        };

        Config {
            network: NetConfig {
                bind_port: net.bind_port.unwrap_or(1234),
                bind_address: net.bind_address.unwrap_or("localhost".to_string()),
                max_connections: net.max_connections.unwrap_or(25),
                accept_ranges: net.accept_ranges.unwrap_or(vec!["*".to_string()]),
            },
            language: LanguageConfig {
                convention: match_convention(lang.convention.unwrap_or("none".to_string()).as_str()),
                force_convention: lang.force_convention.unwrap_or(false),
                database_convention: match_convention(lang.database_convention.unwrap_or("none".to_string()).as_str()),
                force_database_convention: lang.force_database_convention.unwrap_or(false),
                table_convention: match_convention(lang.table_convention.unwrap_or("none".to_string()).as_str()),
                force_table_convention: lang.force_table_convention.unwrap_or(false),
                column_convention: match_convention(lang.column_convention.unwrap_or("none".to_string()).as_str()),
                force_column_convention: lang.force_column_convention.unwrap_or(false),
                procedure_convention: match_convention(lang.procedure_convention.unwrap_or("none".to_string()).as_str()),
                force_procedure_convention: lang.force_procedure_convention.unwrap_or(false),
            },
            logs: LogConfig {
                path: log.path.unwrap_or("/var/lib/kalavar/logs".to_string()),
                debug: log.debug.unwrap_or(true),
                info: log.info.unwrap_or(true),
                log: log.log.unwrap_or(true),
                warn: log.warn.unwrap_or(true),
                error: log.error.unwrap_or(true),
                log_to_file: log.log_to_file.unwrap_or(true),
            },
            privacy,
        }
    }
}


/// Utility method for converting the identifiers of naming conventions to an enumerator for use within the conveter system
pub fn match_convention(convention: &str) -> NamingConvention {
    return match convention {
        "UpperCamelCase" => NamingConvention::UpperCamelCase,
        "lowerCamelCase" => NamingConvention::LowerCamelCase,
        "PascalCase" => NamingConvention::PascalCase,
        "UPPER_SNAKE_CASE" => NamingConvention::UpperSnakeCase,
        "lower_snake_case" => NamingConvention::LowerSnakeCase,
        "kebab-case" => NamingConvention::KebabCase,
        _ => NamingConvention::None
    };
}