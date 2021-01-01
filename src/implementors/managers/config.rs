use crate::managers::config::ConfigManager;
use crate::managers::logging::LoggingManager;
use tokio::fs::{File, create_dir_all};
use tokio::io;
use tokio::io::{AsyncReadExt, ErrorKind, AsyncWriteExt};
use std::string::FromUtf8Error;
use crate::errors::ErrorMap::*;

impl ConfigManager {
    pub async fn new(logger: &mut LoggingManager, os: &str) -> ConfigManager {
        let mut manager = ConfigManager {
            config_path: "".to_string(),
            bind_port: 1234,
            bind_addr: "127.0.0.1".to_string(),
            max_connections: 0,
            connections_per_thread: 5,
            max_threads: 10,
        };

        match os {
            "linux" | "macos" => {
                logger.debug_message("Readjusting configuration path directories for Linux").await;
                manager.config_path = "/etc/kalavar/server.conf".to_string()
            }
            _ => {
                logger.debug_message(format!("OS: {} is un-recognized", os)).await;
                logger.error("Unknown operating system", E202).await;
                logger.fatal("Exiting gracefully", E202, 1).await;
            }
        }

        let mut file: io::Result<File> = File::open(&manager.config_path).await;

        if file.is_ok() {
            let mut inner = file.unwrap();
            manager = parse_config(logger, manager, &mut inner).await;
        } else {
            let err = file.unwrap_err();
            match err.kind() {
                ErrorKind::NotFound => {
                    logger.debug_message("Configuration file not found, generating...").await;
                    file = File::create(&manager.config_path).await;
                    if file.is_ok() {
                        let mut inner: File = file.unwrap();
                        let write_result: io::Result<usize> = inner.write(BASE_CONFIG.as_bytes()).await;
                        if write_result.is_ok() {
                            manager = parse_config(logger, manager, &mut inner).await
                        } else {
                            logger.error(format!("Unable to write to file '{}'", &manager.config_path), E201).await
                        }
                    } else {
                        match file.unwrap_err().kind() {
                            ErrorKind::NotFound => {
                                logger.warn(format!("Directory not found: \"{}\"", manager.config_path)).await;
                                let mut path: Vec<&str> = (&manager.config_path).split("/").collect();
                                path.pop();
                                let result: io::Result<()> = create_dir_all(path.join("/")).await;
                                if result.is_ok() {
                                    file = File::create(&manager.config_path).await;
                                    if file.is_ok() {
                                        let mut inner: File = file.unwrap();
                                        let _ = inner.write(BASE_CONFIG.as_bytes()).await;
                                        manager = parse_config(logger, manager, &mut inner).await
                                    } else {
                                        let e = result.unwrap_err();
                                        match e.kind() {
                                            ErrorKind::PermissionDenied => {
                                                logger.info("You can fix the problem below by running the program using Super User (sudo)").await;
                                                logger.fatal(e, E201, 1).await;
                                            }
                                            _ => {
                                                logger.fatal(e, EXXX, 1).await;
                                            }
                                        }
                                    }
                                } else {
                                    let e = result.unwrap_err();
                                    match e.kind() {
                                        ErrorKind::PermissionDenied => {
                                            logger.info("You can fix the problem below by running the program using Super User (sudo)").await;
                                            logger.fatal(e, E201, 1).await;
                                        }
                                        _ => {
                                            logger.fatal(e, EXXX, 1).await;
                                        }
                                    }
                                }
                            }
                            _ => {
                                logger.error(err, EXXX).await;
                            }
                        }
                    }
                }
                _ => {
                    logger.error(err, EXXX).await;
                }
            }
        }

        manager
    }
}

async fn parse_config(l: &mut LoggingManager, mut m: ConfigManager, file: &mut File) -> ConfigManager {
    let mut content: Vec<u8> = vec!();
    let read_result: io::Result<usize> = file.read_to_end(&mut content).await;

    if read_result.is_ok() {
        let text_result: Result<String, FromUtf8Error> = String::from_utf8(content);
        if text_result.is_ok() {
            let text: String = text_result.unwrap();
            let lines: Vec<&str> = text.split(LINE_ENDING).collect();
            for line in lines {
                    if line.len() > 0 && line.as_bytes()[0] != b"#"[0] && line.contains("=") {
                        let parts: Vec<&str> = line.split("=").collect();
                        if parts.len() == 2 {
                            let key = parts[0];
                            let value = parts[1];
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
            l.error(text_result.unwrap_err(), EXXX).await;
            l.debug_message(&m.config_path).await;
            l.fatal("Invalid config file content, exiting gracefully", EXXX, 1).await;
        }
    } else {
        l.error(read_result.unwrap_err(), EXXX).await;
        l.debug_message(&m.config_path).await;
        l.fatal("Unable to locate config file, exiting gracefully", EXXX, 1).await;
    }

    m
}


const BASE_CONFIG: &str = "# This is the default, automatically generated configuration file.\n# This file was created because no config file was detected on your system at the time the program launched\n\n\n# The port to bind the connection listener to\nport=1234\n
# The IP address to bind the connection listener to\naddress=127.0.0.1\n\n# Which levels of logging should be enabled\ndebug=true\ninfo=true\nlog=true\nwarn=true\nerror=true\n\n# Maximum number of connections to allow\n# Inifinite allows an unlimited amount of connections\n# Automatically calculated from `thread` and `threadcount` values if not present\nconnections=infinite\n
# Maximum number of connections per thread\nthread=5\n\n# Maximum number of threads available to the database
threadcount=10";

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";