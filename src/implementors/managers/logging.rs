use std::fmt::{Debug, Display};

use chrono::{DateTime, Local};

use crate::managers::logging::LoggingManager;
use std::collections::HashMap;
use std::process::exit;

const RESET: &str = "\x1b[0m";
const FG_RED: &str = "\x1b[31m";
const FG_YEL: &str = "\x1b[33m";
const FG_GRE: &str = "\x1b[32m";
const FG_CYA: &str = "\x1b[36m";
const FG_MAG: &str = "\x1b[35m";
const FG_GRY: &str = "\x1b[2m\x1b[37m";

impl LoggingManager {
    pub fn new() -> LoggingManager {
        let mut man = LoggingManager {
            levels: HashMap::new()
        };

        man.levels.insert("DEBUG".to_string(), true);
        man.levels.insert("INFO".to_string(), true);
        man.levels.insert("LOG".to_string(), true);
        man.levels.insert("WARN".to_string(), true);
        man.levels.insert("ERROR".to_string(), true);

        man
    }

    pub fn fatal<A: Display>(&self, content: A, code: i32) {
        println!(" {}FATAL{} > {}{}{} > {}{}{}", FG_RED, RESET, FG_MAG, format_date(), RESET, FG_RED, content, RESET);
        exit(code);
    }

    pub fn debug_message<A: Into<String>>(&self, content: A) {
        if self.levels.get("DEBUG").is_some() {
            println!(" {}DEBUG{} > {}{}{} > {}{}{}", FG_GRE, RESET, FG_MAG, format_date(), RESET, FG_GRE, content.into(), RESET)
        }
    }

    pub fn debug<A: Debug>(&self, content: A) {
        if self.levels.get("DEBUG").is_some() {
            println!(" {}DEBUG{} > {}{}{} > {}{:?}{}", FG_GRE, RESET, FG_MAG, format_date(), RESET, FG_GRE, content, RESET)
        }
    }

    pub fn debug_pretty<A: Debug>(&self, content: A) {
        if self.levels.get("DEBUG").is_some() {
            let l = format!("{:#?}", content);
            for line in l.split("\n").collect::<Vec<&str>>() {
                println!(" {}DEBUG{} > {}{}{} > {}{}{}", FG_GRE, RESET, FG_MAG, format_date(), RESET, FG_GRE, line, RESET)
            }
        }
    }

    pub fn info<A: Display>(&self, content: A) {
        if self.levels.get("INFO").is_some() {
            println!(" {}INFO {} > {}{}{} > {}", FG_CYA, RESET, FG_MAG, format_date(), RESET, content)
        }
    }
    pub fn log<A: Display>(&self, content: A) {
        if self.levels.get("LOG").is_some() {
            println!(" {} LOG {} > {}{}{} > {}", FG_GRY, RESET, FG_MAG, format_date(), RESET, content)
        }
    }

    pub fn warn<A: Display>(&self, content: A) {
        if self.levels.get("WARN").is_some() {
            println!(" {}WARN {} > {}{}{} > {}{}{}", FG_YEL, RESET, FG_MAG, format_date(), RESET, FG_YEL, content, RESET)
        }
    }

    pub fn error<A: Display>(&self, content: A) {
        if self.levels.get("ERROR").is_some() {
            println!(" {}ERROR{} > {}{}{} > {}{}{}", FG_RED, RESET, FG_MAG, format_date(), RESET, FG_RED, content, RESET)
        }
    }
}

pub fn format_date() -> String {
    let now: DateTime<Local> = chrono::Local::now();

    now.format("%Y %b %d - %T").to_string()
}