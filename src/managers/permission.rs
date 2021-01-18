use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// Describes the permissions <user> has for any given database or table
pub struct PermissionManager {
    pub inner: HashMap<String, DatabasePermissions>,
}

// Describes the permissions <user> has for the reference database. Calculated via bitwise operations
pub struct DatabasePermissions {
    pub int: u32,
    pub create: bool,
    pub delete: bool,
    pub read: bool,
    pub write: bool,
    pub change_name: bool,
    pub add_tables: bool,
    pub remove_tables: bool,
    pub modify_tables: bool,
    pub access: HashMap<String, TablePermissions>,
}

// Describes the permissions <user> has for the reference table. Calculated via bitwise operations
pub struct TablePermissions {
    pub int: u32,
    pub create: bool,
    pub delete: bool,
    pub read: bool,
    pub write: bool,
    pub change_name: bool,
    pub add_columns: bool,
    pub remove_columns: bool,
    pub modify_columns: bool,
}

impl Display for PermissionManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut content = String::new();
        for (name, db) in &self.inner {
            content = format!("{}Permissions for '{}':", content, name);
            content = format!("{}\nCREATE:        {}", content, db.create);
            content = format!("{}\nDELETE:        {}", content, db.delete);
            content = format!("{}\nREAD:          {}", content, db.read);
            content = format!("{}\nWRITE:         {}", content, db.write);
            content = format!("{}\nCHANGE NAME:   {}", content, db.change_name);
            content = format!("{}\nADD TABLES:    {}", content, db.add_tables);
            content = format!("{}\nREMOVE TABLES: {}", content, db.remove_tables);
            content = format!("{}\nMODIFY TABLES: {}", content, db.modify_tables);
            content = format!("{}\nNumeric:       {}", content, db.int);

            for (table_name, table) in &db.access {
                content = format!("{}\nPermissions for '{}'.'{}':", content, name, table_name);
                content = format!("{}\nCREATE:        {}", content, table.create);
                content = format!("{}\nDELETE:        {}", content, table.delete);
                content = format!("{}\nREAD:          {}", content, table.read);
                content = format!("{}\nWRITE:         {}", content, table.write);
                content = format!("{}\nCHANGE NAME:   {}", content, table.change_name);
                content = format!("{}\nADD TABLES:    {}", content, table.add_columns);
                content = format!("{}\nREMOVE TABLES: {}", content, table.remove_columns);
                content = format!("{}\nMODIFY TABLES: {}", content, table.modify_columns);
                content = format!("{}\nNumeric:       {}", content, table.int);
            }
        }

        write!(f, "{}", content)

    }
}