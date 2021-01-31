use crate::managers::storage::StorageManager;
use std::collections::HashMap;
use std::time::Instant;
use crate::managers::logging::LoggingManager;
use tokio::fs::{create_dir_all, read_dir, File, ReadDir, OpenOptions};
use tokio::io;
use tokio::io::{ErrorKind, AsyncWriteExt};
use crate::errors::ErrorMap::*;
use crate::core_structures::table_record::TableRecord;
use crate::core_structures::database_record::DatabaseRecord;
use crate::core_structures::column::{ColumnType, ColumnTypeEnum};
use crate::core_structures::row_record::RowRecord;
use crate::core_structures::row::Row;
use rcgen::generate_simple_self_signed;

impl StorageManager {
    /// Instantiates a new storage manager on behalf of the caller
    pub async fn new(l: &mut LoggingManager, os: &str) -> StorageManager {
        let mut storage_manager = StorageManager {
            databases: HashMap::new(),
            last_write: Instant::now(),
            dir: match os {
                "linux" | "macos" => "/var/lib/kalavar".to_string(),
                "windows" => "C:".to_string(),
                _ => {
                    l.fatal("Unable to choose data directory for unknown operating system", GXXX, 1).await;
                    "unknown".to_string()
                }
            },
        };

        l.log_file_full = format!("{}{}", storage_manager.dir, l.log_file);

        let directory_test_result: io::Result<ReadDir> = read_dir(&storage_manager.dir).await;

        if directory_test_result.is_ok() {
            handle_missing_data_dir(l, ErrorKind::NotFound, &storage_manager.dir).await;
            storage_manager = parse_incoming(storage_manager, l).await;
        } else {
            handle_missing_data_dir(l, directory_test_result.unwrap_err().kind(), &storage_manager.dir).await;
            storage_manager = parse_incoming(storage_manager, l).await;
        }

        storage_manager
    }
}

/// Utility method to parse the database files within the data directory
async fn parse_incoming(s: StorageManager, l: &LoggingManager) -> StorageManager {
    let mut failed: (bool, ErrorKind) = (false, ErrorKind::Other);
    let core_files: [&str; 3] = ["/data/k_core/map.kgb", "/data/k_core/map.kdb", l.log_file.as_str()];
    for dir in core_files.iter() {
        if !failed.0 {
            let result = File::create(format!("{}{}", &s.dir, dir)).await;
            if let Ok(mut file) = result {
                l.debug_message(format!("Generated: {}{}", &s.dir, dir)).await;
                if dir.contains(".kgb") {
                    let mut core = DatabaseRecord::new("k_core".to_string(), format!("{}/data/k_core/map.kdb", &s.dir));
                    core.new_table("users".to_string(), vec![("name".to_string(), ColumnType::new(ColumnTypeEnum::String, None, 100)), ("key".to_string(), ColumnType::new_prv(ColumnTypeEnum::String, None, 100)), ("permissions".to_string(), ColumnType::new_prv(ColumnTypeEnum::Array, Some(ColumnTypeEnum::Integer8), 8))]);

                    let mut user_file = OpenOptions::new().create(true).write(true).append(true).open(format!("{}/users/root", &s.dir)).await;
                    let mut root_file = OpenOptions::new().create(true).write(true).append(true).open(format!("{}/kdb.root", dirs::home_dir().unwrap().to_str().unwrap())).await;

                    let cert = generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
                    let public = hex::encode(cert.serialize_pem().unwrap());

                    if let Ok(mut user) = user_file {
                        if let Ok(mut root) = root_file {
                            user.write_all(public.as_bytes()).await;
                            user.flush().await;
                            root.write_all(cert.serialize_private_key_pem().as_bytes()).await;
                            root.flush().await;
                            l.warn(format!("The keyfile for the root user is currently located in {}/kdb.root", dirs::home_dir().unwrap().to_str().unwrap())).await;
                        }
                    }

                    let mut name_col = ColumnType::new(ColumnTypeEnum::String, None, 100);
                    let mut key_col = ColumnType::new_prv(ColumnTypeEnum::String, None, 100);
                    let mut permissions_col = ColumnType::new_prv(ColumnTypeEnum::Array, Some(ColumnTypeEnum::Integer8), 8);


                    let table = core.tables.get_mut("users").unwrap();


                    table.rows.insert(0, RowRecord {
                        inner: Row::new(vec![("root".to_string(), name_col), (public, key_col), ("255".to_string(), permissions_col)]),
                        start: 0,
                        end: 0,
                        id: 0,
                        backing: "".to_string(),
                        backing_is_dir: false,
                        columns: Default::default(),
                    });
                    let core_res = bincode::serialize(&core);
                    if let Ok(core_b) = core_res {
                        file.write_all(core_b.as_slice()).await;
                        file.flush().await;
                    } else {
                        dbg!(core_res);
                    }
                }
            } else {
                l.error(format!("Failed to generate file: {}{}", &s.dir, dir), G201).await;
                failed = (true, result.unwrap_err().kind());
            }
        }
    }

    if failed.0 {
        l.error(format!("Error type: {:?}", failed.1), G201).await;
        l.fatal("Unable to create required core files, please run the program as an administrator to generate them.", G201, 1).await;
    } else {
        l.debug_message("Parsing files").await;
    }
    s
}

/// Utility function to handle the event that the data directory does not exist in a more graceful manner than just panicking
async fn handle_missing_data_dir(l: &LoggingManager, e: ErrorKind, root: &String) {
    match e {
        ErrorKind::NotFound => {
            l.warn("Data directory not located, attempting to generate core information...").await;

            let base_dirs: [&str; 6] = ["", "/users", "/core", "/logs", "/data", "/data/k_core"];
            let mut failed: (bool, ErrorKind) = (false, ErrorKind::Other);
            for dir in base_dirs.iter() {
                if !failed.0 {
                    let result = create_dir_all(format!("{}{}", root, dir)).await;
                    if result.is_ok() {
                        l.debug_message(format!("Generated: {}{}", root, dir)).await;
                    } else {
                        l.error(format!("Failed to generate directory: {}{}", root, dir), G201).await;
                        failed = (true, result.unwrap_err().kind());
                    }
                }
            }

            if failed.0 {
                l.error(format!("Error type: {:?}", failed.1), G201).await;
                l.fatal("Unable to create required base directories, please run the program as an administrator to generate them.", G201, 1).await;
            }
        }
        _ => {
            l.error(format!("Error type: {:?}", e), GXXX).await;
        }
    }
}