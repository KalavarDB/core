use crate::managers::storage::StorageManager;
use std::collections::HashMap;
use std::time::Instant;
use crate::managers::logging::LoggingManager;
use tokio::fs::{create_dir_all, read_dir, File, ReadDir};
use tokio::io;
use tokio::io::ErrorKind;

impl StorageManager {
    pub async fn new(l: &LoggingManager, os: &str) -> StorageManager {
        let mut storage_manager = StorageManager {
            databases: HashMap::new(),
            last_write: Instant::now(),
            dir: match os {
                "linux" | "macos" => "/var/lib/kalavar".to_string(),
                "windows" => "C:".to_string(),
                _ => {
                    l.fatal("Unable to choose data directory for unknown operating system", 1);
                    "unknown".to_string()
                }
            }
        };


        let directory_test_result: io::Result<ReadDir> = read_dir(&storage_manager.dir).await;

        if directory_test_result.is_ok() {
            storage_manager = parse_incoming(storage_manager.clone()).await;
        } else {
            handle_missing_data_dir(l, directory_test_result.unwrap_err().kind(), &storage_manager.dir).await;
            storage_manager = parse_incoming(storage_manager.clone()).await;
        }

        storage_manager
    }
}

async fn parse_incoming(s: StorageManager) -> StorageManager {
    s
}


async fn handle_missing_data_dir(l: &LoggingManager, e: ErrorKind, root: &String) {
    match e {
        ErrorKind::NotFound => {
            l.warn("Data directory not located, attempting to generate core information...");

            let base_dirs: [&str;4] = ["", "/core", "/logs", "/data"];
            let mut failed: (bool, ErrorKind) = (false, ErrorKind::Other);
            for dir in base_dirs.iter() {
                if !failed.0 {
                    let result = create_dir_all(format!("{}{}", root, dir)).await;
                    if result.is_ok() {
                        l.debug_message(format!("Generated: {}{}", root, dir));
                    } else {
                        l.error(format!("Failed to generate directory: {}{}", root, dir));
                        failed = (true, result.unwrap_err().kind());
                    }
                }
            }

            if failed.0 {
                l.error(format!("Error type: {:?}", failed.1));
                l.fatal("Unable to create required base directories, please run the program as an administrator to generate them.", 1);
            } else {
                // generate root files
            }

            let core_files: [&str;4] = ["", "/", "/logs", "/data"];
            for dir in core_files.iter() {
                if !failed.0 {
                    let result = File::create(format!("{}{}", root, dir)).await;
                    if result.is_ok() {
                        l.debug_message(format!("Generated: {}{}", root, dir));
                    } else {
                        l.error(format!("Failed to generate directory: {}{}", root, dir));
                        failed = (true, result.unwrap_err().kind());
                    }
                }
            }

            if failed.0 {
                l.error(format!("Error type: {:?}", failed.1));
                l.fatal("Unable to create required base directories, please run the program as an administrator to generate them.", 1);
            } else {
                // generate root files
            }


        }
        _ => {
            l.error(format!("Error type: {:?}", e));
        }
    }

}