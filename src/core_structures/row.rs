use crate::core_structures::column::ColumnType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Row {
    pub entries: HashMap<String, Cell>
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub inner_type: ColumnType,
    pub inner_value: Box<[u8]>
}