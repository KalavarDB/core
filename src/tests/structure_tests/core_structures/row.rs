use crate::core_structures::row::{Row, Cell};
use crate::core_structures::column::{ColumnType, ColumnTypeEnum};
use crate::core_structures::as_bytes::AsBytes;

#[test]
fn test_new_row() {
    let mut c = ColumnType::new_prv(ColumnTypeEnum::Integer64, None, 64);
    let mut r = Row::new(vec![("__IDENTIFIER__".to_string(), c)]);

    r.entries.get_mut("__IDENTIFIER__").unwrap().inner_value = 64.as_kv_bytes();

    assert!(r.entries.contains_key("__IDENTIFIER__") && r.entries.get("__IDENTIFIER__").unwrap().inner_type.is_private && r.entries.get("__IDENTIFIER__").unwrap().inner_value == 64.as_kv_bytes());
}
