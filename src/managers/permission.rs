use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Describes the permissions <user> has for any given database or table
pub struct PermissionManager {
    /// A map containing keyed database permission sets, where the key is the name of the database in question
    pub inner: HashMap<String, DatabasePermissions>,
}

/// Describes the permissions <user> has for the reference database. Calculated via bitwise operations
pub struct DatabasePermissions {
    /// The permissions integer used to generate this permission tree
    pub int: u32,

    /// If the user can create databases
    pub create: bool,

    /// If the user can delete databases
    pub delete: bool,

    /// If the user can read the content of the database
    pub read: bool,

    /// If the user is able to write to the database (modifying it)
    pub write: bool,

    /// If the user can change the name of the database
    pub change_name: bool,

    /// If the user can add tables to the database
    pub add_tables: bool,

    /// If the user can remove tables from the database
    pub remove_tables: bool,

    /// If the user can modify the tables within the database
    pub modify_tables: bool,

    /// A map containing keyed table permission sets, where the key is the name of the table in question
    pub access: HashMap<String, TablePermissions>,
}

/// Describes the permissions <user> has for the reference table. Calculated via bitwise operations
pub struct TablePermissions {
    /// The permissions integer used to generate this permission tree
    pub int: u32,

    /// If the user can create tables within the database
    pub create: bool,

    /// If the user can remove tables from the database
    pub delete: bool,

    /// If the user can read the contents of the table in question
    pub read: bool,

    /// If the user is able to write table (modifying it)
    pub write: bool,

    /// If the user is allowed to change the name of the table
    pub change_name: bool,

    /// If the user can add additional columns to the table
    pub add_columns: bool,

    /// If the user can remove columns from the table
    pub remove_columns: bool,

    /// If the user can modify the currently existing columns in the table
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