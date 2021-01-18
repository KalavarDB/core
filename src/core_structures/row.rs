// STD Lib imports
use std::collections::HashMap;

// Internal crate imports
use crate::core_structures::column::ColumnType;
use crate::core_structures::as_bytes::AsBytes;

/// A structure wrapping the cells of each row in a neat little basket
#[derive(Debug, Clone)]
pub struct Row {
    /// A map linking each cell within the row, to its parent column
    pub entries: HashMap<String, Cell>
}

/// A structure defining the type of column that the data is from, as well as the raw bytes of the given data
#[derive(Debug, Clone)]
pub struct Cell {
    /// The type of data stored in "inner_value"
    /// Influences how the "inner_value" is parsed into content
    pub inner_type: ColumnType,

    /// The raw binary data for the given data type, must be parsed to be made useful
    pub inner_value: Vec<u8>,
}

/// # The following content is undocumented due to not being ready for documentation at this time.
/// # You are welcome to attempt to make sense of it though.
impl Row {
    pub fn new(cells: Vec<(String, ColumnType)>) -> Row {
        let mut r = Row {
            entries: HashMap::new()
        };

        for c in cells {
            r.entries.insert(c.0, Cell {
                inner_type: c.1,
                inner_value: Vec::new(),
            });
        }

        r
    }

    pub async fn populate<A: AsBytes>(&mut self, column: String, content: A) {
        let previous = self.entries.remove_entry(column.as_str()).unwrap();
        self.entries.insert(column, Cell {
            inner_type: previous.1.inner_type,
            inner_value: content.as_kv_bytes(),
        });
    }
}


