use std::collections::HashMap;
use std::fmt::Display;
use serde::export::Formatter;

// Describes the permissions <user> has for any given database or table
pub struct PermissionManager {
    pub inner: HashMap<String, DatabasePermissions>,
}

// Describes the permissions <user> has for the reference database. Calculated via bitwise operations
pub struct DatabasePermissions {
    pub int: u32,
    pub create: bool,
    pub delete: bool,
    pub write: bool,
    pub change_name: bool,
    pub add_columns: bool,
    pub remove_columns: bool,
    pub change_columns: bool,
    pub read: bool,
    pub access: HashMap<String, TablePermissions>,
}

// Describes the permissions <user> has for the reference table. Calculated via bitwise operations
pub struct TablePermissions {
    pub int: u32,
    pub create: bool,
    pub delete: bool,
    pub write: bool,
    pub change_name: bool,
    pub add_columns: bool,
    pub remove_columns: bool,
    pub change_columns: bool,
    pub read: bool,
}

impl Display for PermissionManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {

        for (name, db) in self.inner {
            write!(f, "{}", name);

            for (name, table) in db.access {
                write
            }
        }

        write!(f, "\n")
    }
}