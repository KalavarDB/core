use crate::managers::storage::StorageManager;
use std::collections::HashMap;
use std::time::Instant;
use crate::managers::logging::LoggingManager;
use tokio::fs::{create_dir_all, read_dir, File, ReadDir, OpenOptions};
use tokio::io;
use tokio::io::{ErrorKind, AsyncWriteExt};
use crate::errors::ErrorMap::*;
use crate::core_structures::{
    database_record::DatabaseRecord,
    column::{
        ColumnType,
        ColumnTypeEnum,
    },
    row_record::RowRecord,
    row::Row,
};
use rcgen::generate_simple_self_signed;
use std::env::consts::OS;
use tokio::fs::DirEntry;
use std::io::Result;

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
                    l.fatal("Unable to choose data directory for unknown operating system", G000, 1).await;
                    "unknown".to_string()
                }
            },
            map: HashMap::new(),
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
async fn parse_incoming(mut s: StorageManager, l: &mut LoggingManager) -> StorageManager {
    let mut failed: (bool, ErrorKind) = (false, ErrorKind::Other);
    let core_files: [&str; 3] = ["/data/k_core/map.kgb", "/data/k_core/map.kdb", l.log_file.as_str()];
    for dir in core_files.iter() {
        if !failed.0 {
            let exists = File::open(format!("{}{}", &s.dir, dir)).await;
            if let Err(e) = exists {
                match e.kind() {
                    ErrorKind::NotFound => {
                        let result = File::create(format!("{}{}", &s.dir, dir)).await;
                        if let Ok(mut file) = result {
                            l.debug_message(format!("Generated: {}{}", &s.dir, dir)).await;
                            if dir.contains(".kgb") {
                                let mut core = DatabaseRecord::new("k_core".to_string(), format!("{}/data/k_core/map.kdb", &s.dir));
                                core.new_table("users".to_string(), vec![("name".to_string(), ColumnType::new(ColumnTypeEnum::String, None, 100)), ("key".to_string(), ColumnType::new_prv(ColumnTypeEnum::String, None, 100)), ("permissions".to_string(), ColumnType::new_prv(ColumnTypeEnum::Array, Some(ColumnTypeEnum::Integer8), 8))]);

                                let mut user = std::env::var("USER").unwrap_or("".to_string());
                                let mut home_dir = "/home/".to_string();
                                if OS == "linux" || OS == "darwin" {
                                    if user == "root" || user == "" {
                                        let u2 = std::env::var("SUDO_USER").unwrap_or("".to_string());
                                        if u2 == "" {
                                            l.fatal("Unable to determine home directory of user", G102, 1).await;
                                        } else {
                                            user = u2;
                                        }
                                    }
                                    home_dir = format!("{}{}", home_dir, user);
                                } else {
                                    home_dir = dirs::home_dir().unwrap().to_string_lossy().to_string();
                                }

                                l.debug(&home_dir).await;

                                let mut user_file = OpenOptions::new().create(true).write(true).append(false).open(format!("{}/users/root", &s.dir)).await;
                                let mut root_file = OpenOptions::new().create(true).write(true).append(false).open(format!("{}/kdb.root", home_dir)).await;

                                let cert = generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
                                let public = hex::encode(cert.serialize_pem().unwrap());

                                if let Ok(mut user) = user_file {
                                    if let Ok(mut root) = root_file {
                                        user.write_all(public.as_bytes()).await;
                                        user.flush().await;
                                        root.write_all(cert.serialize_private_key_pem().as_bytes()).await;
                                        root.flush().await;
                                        l.warn(format!("The keyfile for the root user is currently located in {}/kdb.root", home_dir)).await;
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
                                    dbg!(core_res.unwrap_err());
                                }
                            }
                        } else {
                            l.error(format!("Failed to generate file: {}{}", &s.dir, dir), G201).await;
                            failed = (true, result.unwrap_err().kind());
                        }
                    }
                    _ => {
                        panic!(e)
                    }
                }
            } else {
                l.info(format!("core file already exists {}", format!("{}{}", &s.dir, dir)));
            }
        }
    }

    if failed.0 {
        l.error(format!("Error type: {:?}", failed.1), G201).await;
        l.fatal("Unable to create required core files, please run the program as an administrator to generate them.", G201, 1).await;
    } else {
        l.debug_message("Parsing files").await;
        parse_databases(&mut s, &l).await;
    }
    s
}

/// Function to help with parsing the contents of the database into real-usable data
async fn parse_databases(s: &mut StorageManager, l: &LoggingManager) {
    let mut files: Vec<(String, Result<File>)> = vec![];
    let mut peeked_directory: Vec<DirEntry> = peek_dir(format!("{}/data", s.dir), l).await;

    let mut index = 0;
    let mut counter = peeked_directory.len();


    while index < counter {
        let file = &peeked_directory[index];
        let path = file.path().to_string_lossy().to_string();
        if file.metadata().await.unwrap().is_dir() {
            let mut sub = peek_dir(path, l).await;
            peeked_directory.append(&mut sub);
            counter = peeked_directory.len();
        } else {
            files.push((path.clone(), OpenOptions::new().read(true).write(true).append(false).open(path).await));
        }
        index += 1;
    }


    for (path, possible_file) in files {
        if let Ok(file) = possible_file {
            l.debug(file).await;
            // TODO: work on making the file handle accessible via the storage manager, as well as initially parsing the file into a DatabaseRecord
        } else {
            match possible_file.unwrap_err().kind() {
                ErrorKind::NotFound => {
                    l.error(format!("Unable to open file \"{}\" because the file could not be found", path), D503).await;
                }
                ErrorKind::PermissionDenied => {
                    l.error(format!("Unable to open file \"{}\" due to a permissions error", path), D504).await;
                }
                ErrorKind::WouldBlock => {
                    l.error(format!("Unable to open file \"{}\" because doing so would block the system I/O", path), D505).await;
                }
                ErrorKind::TimedOut => {
                    l.error(format!("Unable to open file \"{}\" because the system timed out", path), D506).await;
                }
                ErrorKind::Interrupted => {
                    l.error(format!("Unable to open file \"{}\" because something interrupted the request", path), D507).await;
                }
                ErrorKind::UnexpectedEof => {
                    l.error(format!("Unable to open file \"{}\" because it ended unexpectedly", path), D508).await;
                }
                _ => {
                    l.error(format!("Unable to open file \"{}\" due to an unknown error", path), D509).await;
                }
            }
        }
    }
}

/// Utility function to return the files from a directory
async fn peek_dir(path: String, l: &LoggingManager) -> Vec<DirEntry> {
    let mut paths: Vec<DirEntry> = vec![];
    let file_res = read_dir(path).await;

    if let Ok(mut files) = file_res {
        let mut dir = files.next_entry().await.unwrap();
        while dir.is_some() {
            paths.push(dir.unwrap());
            dir = files.next_entry().await.unwrap();
        }
    }

    paths
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
            l.error(format!("Error type: {:?}", e), G000).await;
        }
    }
}