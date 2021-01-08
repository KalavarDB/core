// STD Lib imports
use std::fmt::{Debug, Display};
use std::collections::HashMap;
use std::process::exit;


// External crate imports
use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use uuid::Uuid;
use chrono::{DateTime, Local};

// Internal crate imports
use crate::managers::logging::LoggingManager;


// CONST defined color escape strings for the terminal output
// These only work for terminals supporting the ANSI escape standard colors
const RESET: &str = "\x1b[0m";
const FG_RED: &str = "\x1b[31m";
const FG_YEL: &str = "\x1b[33m";
const FG_GRE: &str = "\x1b[32m";
const FG_CYA: &str = "\x1b[36m";
const FG_MAG: &str = "\x1b[35m";
const FG_GRY: &str = "\x1b[2m\x1b[37m";

impl LoggingManager {
    // Generates a new (fully populated) instance of the logging manager
    pub fn new() -> LoggingManager {
        // Generate a new id number for the log to minimise collisions
        let log_id = Uuid::new_v4().to_string();

        // Splits the log_id at it's "-" chars to get the first entry as the log id
        let comps: Vec<&str> = log_id.split("-").collect();

        // Builds the manager with the available information
        let mut man = LoggingManager {
            levels: HashMap::new(),
            log_file: format!("/logs/{}.log", comps[0]),
            log_file_full: format!("/logs/{}.log", comps[0]),
        };

        // The following 5 lines of code insert the respectively allowed logging levels.
        // These are then removed automatically by the configuration manager via mutable referencing
        man.levels.insert("DEBUG".to_string(), true);
        man.levels.insert("INFO".to_string(), true);
        man.levels.insert("LOG".to_string(), true);
        man.levels.insert("WARN".to_string(), true);
        man.levels.insert("ERROR".to_string(), true);

        // Return the manager stored in the "man" variable
        man
    }


    // Log data to the console and the log file with the appropriate escapes and modifications for a FATAL ERROR
    // Forces all ongoing tasks to exit, and quits the program itself
    pub async fn fatal<A: Display>(&self, content: A, error: crate::errors::ErrorMap, code: i32) {
        println!(" {}FATAL{} > {:?} > {}{}{} > {}{}{}", FG_RED, RESET, error, FG_MAG, format_date().await, RESET, FG_RED, content, RESET);

        // Logs relevant information about the error itself using the "info" method
        self.info(format!("You can see more information about error {} by going to", error)).await;
        self.info(format!("https://kalavar.cf/documentation/errors/{}", error)).await;

        // Attempts to open the logging file
        let file = OpenOptions::new().write(true).append(true).open(&self.log_file_full).await;

        match file {
            Ok(mut f) => {
                f.write_all(format!(" FATAL > {:?} > {} > {}\n", error, format_date().await, content).as_bytes()).await;
            }
            _ => {}
        }
        exit(code);
    }

    // Logs content to the terminal and file with the "DEBUG" notification, but without any decoration
    pub async fn debug_message<A: Into<String>>(&self, content: A) {
        if self.levels.get("DEBUG").is_some() {
            let c = content.into();
            println!(" {}DEBUG{} >      > {}{}{} > {}{}{}", FG_GRE, RESET, FG_MAG, format_date().await, RESET, FG_GRE, c, RESET);

            // Attempts to open the logging file
            let file = OpenOptions::new().write(true).append(true).open(&self.log_file_full).await;

            match file {
                Ok(mut f) => {
                    f.write_all(format!(" DEBUG >      > {} > {}\n", format_date().await, c).as_bytes()).await;
                }
                _ => {}
            }
        }
    }

    // Prints content to the terminal and file without any pretty formatting
    pub async fn debug<A: Debug>(&self, content: A) {
        if self.levels.get("DEBUG").is_some() {
            println!(" {}DEBUG{} >      > {}{}{} > {}{:?}{}", FG_GRE, RESET, FG_MAG, format_date().await, RESET, FG_GRE, content, RESET);

            // Attempts to open the logging file
            let file = OpenOptions::new().write(true).append(true).open(&self.log_file_full).await;

            match file {
                Ok(mut f) => {
                    f.write_all(format!(" DEBUG >      > {} > {:?}\n", format_date().await, content).as_bytes()).await;
                }
                _ => {}
            }
        }
    }


    // Prints a single entry to the terminal and console for every line necessary to pretty print a debug message in full
    pub async fn debug_pretty<A: Debug>(&self, content: A) {
        if self.levels.get("DEBUG").is_some() {
            let l = format!("{:#?}", content);

            // Loop over every line in a debug message and print a single entry accordingly
            for line in l.split("\n").collect::<Vec<&str>>() {
                println!(" {}DEBUG{} >      > {}{}{} > {}{}{}", FG_GRE, RESET, FG_MAG, format_date().await, RESET, FG_GRE, line, RESET);

                // Attempts to open the logging file
                let file = OpenOptions::new().write(true).append(true).open(&self.log_file_full).await;

                match file {
                    Ok(mut f) => {
                        f.write_all(format!(" DEBUG >      > {} > {}\n", format_date().await, line).as_bytes()).await;
                    }
                    _ => {}
                }
            }
        }
    }

    // Displays information to the user via the terminal and log file
    pub async fn info<A: Display>(&self, content: A) {
        if self.levels.get("INFO").is_some() {
            println!(" {}INFO {} >      > {}{}{} > {}", FG_CYA, RESET, FG_MAG, format_date().await, RESET, content);

            // Attempts to open the logging file
            let file = OpenOptions::new().write(true).append(true).open(&self.log_file_full).await;

            match file {
                Ok(mut f) => {
                    f.write_all(format!(" INFO  >      > {} > {}\n", format_date().await, content).as_bytes()).await;
                }
                _ => {}
            }
        }
    }

    // Prints information to the terminal and log file
    pub async fn log<A: Display>(&self, content: A) {
        if self.levels.get("LOG").is_some() {
            println!(" {} LOG {} >      > {}{}{} > {}", FG_GRY, RESET, FG_MAG, format_date().await, RESET, content);

            // Attempts to open the logging file
            let file = OpenOptions::new().write(true).append(true).open(&self.log_file_full).await;

            match file {
                Ok(mut f) => {
                    f.write_all(format!("  LOG  >      > {} > {}\n", format_date().await, content).as_bytes()).await;
                }
                _ => {}
            }
        }
    }

    // Prints a warning to the terminal and log file
    pub async fn warn<A: Display>(&self, content: A) {
        if self.levels.get("WARN").is_some() {
            println!(" {}WARN {} >      > {}{}{} > {}{}{}", FG_YEL, RESET, FG_MAG, format_date().await, RESET, FG_YEL, content, RESET);
            // Attempts to open the logging file
            let file = OpenOptions::new().write(true).append(true).open(&self.log_file_full).await;

            match file {
                Ok(mut f) => {
                    f.write_all(format!(" WARN  >      > {} > {}\n", format_date().await, content).as_bytes()).await;
                }
                _ => {}
            }
        }
    }

    // Notifies the user of an error via a message in the log file and on the terminal
    pub async fn error<A: Display>(&self, content: A, error: crate::errors::ErrorMap) {
        if self.levels.get("ERROR").is_some() {
            println!(" {}ERROR{} > {:?} > {}{}{} > {}{}{}", FG_RED, RESET, error, FG_MAG, format_date().await, RESET, FG_RED, content, RESET);

            // Logs relevant information about the error itself using the "info" method
            self.info(format!("You can see more information about error {} by going to", error)).await;
            self.info(format!("https://kalavar.cf/documentation/errors/{}", error)).await;

            // Attempts to open the logging file
            let file = OpenOptions::new().write(true).append(true).open(&self.log_file_full).await;

            match file {
                Ok(mut f) => {
                    f.write_all(format!(" ERROR > {:?} > {} > {}\n", error, format_date().await, content).as_bytes()).await;
                }
                _ => {}
            }
        }
    }
}

// Helper method for formatting date-time information for the console and log file outputs.
pub async fn format_date() -> String {
    let now: DateTime<Local> = chrono::Local::now();

    now.format("%Y %b %d - %T").to_string()
}