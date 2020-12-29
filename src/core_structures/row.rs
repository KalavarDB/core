use crate::core_structures::column::ColumnType;
use std::collections::HashMap;
use crate::core_structures::as_bytes::AsBytes;

#[derive(Debug, Clone)]
pub struct Row {
    pub entries: HashMap<String, Cell>
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub inner_type: ColumnType,
    pub inner_value: Vec<u8>
}


impl Row {
    pub fn new(cells: Vec<(String, ColumnType)>) -> Row {
        let mut r = Row {
            entries: HashMap::new()
        };

        for c in cells {
            r.entries.insert(c.0, Cell {
                inner_type: c.1,
                inner_value: Vec::new()
            });
        }

        r
    }

    pub async fn populate<A: AsBytes>(&mut self, column: String, content: A) {

        let previous = self.entries.remove_entry(column.as_str()).unwrap();
        self.entries.insert(column, Cell {
            inner_type: previous.1.inner_type,
            inner_value: content.as_kv_bytes()
        });
    }

}


