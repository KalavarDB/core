// STD Lib imports


// External crate imports
use tokio::fs::{File};
use tokio::io;


// Internal crate imports
use crate::managers::config::{ConfigManager};
use crate::managers::logging::LoggingManager;
use crate::errors::ErrorMap::*;

impl ConfigManager {
    /// A helper method to build and pre-configure a configuration manager structure and return it to the caller
    pub async fn new(logger: &mut LoggingManager, os: &str) -> ConfigManager {

        // Instantiate the manager with pre-defined defaults
        let mut manager = ConfigManager {
            config_path: "".to_string(),
            config: toml::from_str(BASE_CONFIG).unwrap()
        };

        // Attempt to determine the location of the configuration file based on the operating system (and by extension, file system)
        match os {
            // If the operating system is determined to be Linux or MacOS
            // Send a debug message to the terminal about the update to the config path
            // And set the config path accordingly
            "linux" | "macos" => {
                logger.debug_message("Readjusting configuration path directories for Linux").await;
                manager.config_path = "/etc/kalavar/server.toml".to_string()
            }

            // If the operating matches none of the previous branches of this logic tree
            // Report the problem to the user, and exit with a fatal error due to the un-programmed logic for
            // non-unix systems such as windows machines
            _ => {
                logger.debug_message(format!("OS: {} is un-recognized", os)).await;
                logger.error("Unknown operating system", G202).await;
                logger.fatal("Exiting gracefully", G202, 1).await;
            }
        }

        // Attempt to open the config file
        let _file: io::Result<File> = File::open(&manager.config_path).await;

        // If the file is okay, parse the file
        // If it is not, check the type of the error and act accordingly
        // TODO: update the "parse_config" function to istead parse the config from TOML format instead of the current one
        /*
        if file.is_ok() {
            let mut inner = file.unwrap();
            manager = parse_config(logger, manager, &mut inner).await;
        } else {
            let err = file.unwrap_err();

            // Match the type of error against a set of actions for each available error type
            match err.kind() {
                // If the file is not found assume it has not been created, and attempt to generate a fresh one
                ErrorKind::NotFound => {
                    logger.debug_message("Configuration file not found, generating...").await;

                    // Attempt to create the config file
                    file = File::create(&manager.config_path).await;

                    // If the file created successfully, write the default contents of the file automatically
                    // If note, check the type of error and act accordingly
                    if file.is_ok() {
                        let mut inner: File = file.unwrap();

                        // Attempt to write the contents of the default configuration file to the newly created one
                        let write_result: io::Result<usize> = inner.write(BASE_CONFIG.as_bytes()).await;

                        // If the attempt was successful, parse the configuration as normal
                        // If not, display an error in the console and break free from this logic chain
                        if write_result.is_ok() {
                            manager = parse_config(logger, manager, &mut inner).await
                        } else {
                            logger.error(format!("Unable to write to file '{}'", &manager.config_path), G201).await
                        }
                    } else {
                        match file.unwrap_err().kind() {
                            // If the file was not found, assume that the directory is missing, generate the necessary directory
                            ErrorKind::NotFound => {
                                logger.warn(format!("Directory not found: \"{}\"", manager.config_path)).await;
                                let mut path: Vec<&str> = (&manager.config_path).split("/").collect();

                                // Remove the last component of the config path (the "/server.toml" part)
                                // To leave us with the directory tree
                                path.pop();

                                // Attempt to create the directory
                                let result: io::Result<()> = create_dir_all(path.join("/")).await;

                                // If the attempt was a success, create the file as normal and write default config file, and parse as normal
                                // If not, match the type of error, and act accordingly
                                if result.is_ok() {
                                    file = File::create(&manager.config_path).await;
                                    if file.is_ok() {
                                        let mut inner: File = file.unwrap();
                                        let _ = inner.write(BASE_CONFIG.as_bytes()).await;
                                        manager = parse_config(logger, manager, &mut inner).await
                                    } else {
                                        let e = result.unwrap_err();

                                        // Match the type of error to the tree
                                        match e.kind() {
                                            // If the error was as a result of permissions, report it to the console and exit the program using
                                            // Error code G201
                                            ErrorKind::PermissionDenied => {
                                                logger.info("You can fix the problem below by running the program using Super User (sudo)").await;
                                                logger.fatal(e, G201, 1).await;
                                            }

                                            // If the error was not in the prior arms of this logic tree, report it under error code GXXX and
                                            // Exit with a fatal error
                                            _ => {
                                                logger.fatal(e, GXXX, 1).await;
                                            }
                                        }
                                    }
                                } else {
                                    let e = result.unwrap_err();

                                    // Match the type of error to the tree
                                    match e.kind() {
                                        // If the error was as a result of permissions, report it to the console and exit the program using
                                        // Error code G201
                                        ErrorKind::PermissionDenied => {
                                            logger.info("You can fix the problem below by running the program using Super User (sudo)").await;
                                            logger.fatal(e, G201, 1).await;
                                        }

                                        // If the error was not in the prior arms of this logic tree, report it under error code GXXX and
                                        // Exit with a fatal error
                                        _ => {
                                            logger.fatal(e, GXXX, 1).await;
                                        }
                                    }
                                }
                            }

                            // If the error is not in the above logic, exit with a fatal error whilst reporting GXXX to the console
                            _ => {
                                logger.error(err, GXXX).await;
                            }
                        }
                    }
                }

                // If the error is anything else
                // display an error message as well as a (currently undefined) error code for further information
                _ => {
                    logger.error(err, GXXX).await;
                }
            }
        }*/

        // Return the now created and ready to use config manager to the caller
        manager
    }
}


/*
// Helper function to parse the config file and modify the manager accordingly
async fn parse_config(l: &mut LoggingManager, mut m: ConfigManager, file: &mut File) -> ConfigManager {
    let mut content: Vec<u8> = vec!();

    // Attempt to read the config file
    let read_result: io::Result<usize> = file.read_to_end(&mut content).await;

    // If the file was able to be read successfully, parse it accordingly
    // If not, exit with a GXXX error and report a fatal error to the terminal
    if read_result.is_ok() {
        // Attempt to parse the file to utf8 from binary, in case it has been overwritten with a binary format
        let text_result: Result<String, FromUtf8Error> = String::from_utf8(content);

        // If the contents parsed successfully, match each line against a logic tree
        // If not, exit using the GXXX error code and report a fatal error to the terminal
        if text_result.is_ok() {
            let text: String = text_result.unwrap();
            // Split the file according to the system line endings into individual lines
            let lines: Vec<&str> = text.split(LINE_ENDING).collect();

            // Loop through each line of the file
            for line in lines {
                // Check the length of the line is more than 0, and the line is not commented out,
                // and also contains the "=" character to prevent out of bounds errors from the arrays
                if line.len() > 0 && line.as_bytes()[0] != b"#"[0] && line.contains("=") {
                    // Split the line at the "=" character to get the key-value pair as individual strings
                    let parts: Vec<&str> = line.split("=").collect();

                    // If there are exactly 2 parts, match them against a logic tree
                    if parts.len() == 2 {
                        let key = parts[0];
                        let value = parts[1];

                        // Match the key against a logic tree for each of the possible config options available to the program at the current time
                        match key {
                            "connections" => {
                                if value.to_lowercase() == "infinite" {
                                    l.warn("Using 'infinite' connection mode is not advisable, it may lead to severe slowdowns during large queries").await;
                                } else {
                                    let port = value.parse();
                                    if port.is_ok() {
                                        m.max_connections = port.unwrap();
                                    } else {
                                        l.warn("Invalid value specified for the \"connections\" configuration. Should be an integer or 'infinite'").await;
                                    }
                                }
                            }
                            "thread" => {
                                let port = value.parse();
                                if port.is_ok() {
                                    m.connections_per_thread = port.unwrap();
                                } else {
                                    l.warn("Invalid value specified for the \"thread\" configuration. Should be an integer").await;
                                }
                            }
                            "threadcount" => {
                                let port = value.parse();
                                if port.is_ok() {
                                    m.max_threads = port.unwrap();
                                } else {
                                    l.warn("Invalid value specified for the \"threadcount\" configuration. Should be an integer").await;
                                }
                            }
                            "port" => {
                                let port = value.parse();
                                if port.is_ok() {
                                    m.bind_port = port.unwrap();
                                } else {
                                    l.warn("Invalid value specified for the \"port\" configuration. Should be an integer").await;
                                }
                            }
                            "address" => {
                                m.bind_addr = value.to_string();
                            }
                            "debug" => {
                                if value == "false" {
                                    l.levels.remove("DEBUG");
                                }
                            }
                            "info" => {
                                if value == "false" {
                                    l.levels.remove("INFO");
                                }
                            }
                            "log" => {
                                if value == "false" {
                                    l.levels.remove("LOG");
                                }
                            }
                            "warn" => {
                                if value == "false" {
                                    l.levels.remove("WARN");
                                }
                            }
                            "error" => {
                                if value == "false" {
                                    l.levels.remove("ERROR");
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            l.error(text_result.unwrap_err(), GXXX).await;
            l.debug_message(&m.config_path).await;
            l.fatal("Invalid config file content, exiting gracefully", GXXX, 1).await;
        }
    } else {
        l.error(read_result.unwrap_err(), GXXX).await;
        l.debug_message(&m.config_path).await;
        l.fatal("Unable to locate config file, exiting gracefully", GXXX, 1).await;
    }

    m
}*/

// TODO: update `BASE_CONFIG` const to use TOML formatted config file
/// (OBSOLETE) Defines the default configuration file, this has been minimised using the "\n" escape to signify that a newline is intended, but not present to keep the source code length down
const BASE_CONFIG: &str = "# This is the default, automatically generated configuration file.\n# This file was created because no config file was detected on your system at the time the program launched\n\n\n# The port to bind the connection listener to\nport=1234\n
# The IP address to bind the connection listener to\naddress=127.0.0.1\n\n# Which levels of logging should be enabled\ndebug=true\ninfo=true\nlog=true\nwarn=true\nerror=true\n\n# Maximum number of connections to allow\n# Inifinite allows an unlimited amount of connections\n# Automatically calculated from `thread` and `threadcount` values if not present\nconnections=infinite\n
# Maximum number of connections per thread\nthread=5\n\n# Maximum number of threads available to the database
threadcount=10";