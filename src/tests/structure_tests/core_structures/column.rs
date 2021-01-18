use crate::core_structures::column::{ColumnType, ColumnTypeEnum};

#[test]
fn test_new() {
    let c = ColumnType::new(ColumnTypeEnum::Snowflake, None, 0);
    assert_eq!(c.is_private, false)
}

#[test]
fn test_new_private() {
    let c = ColumnType::new_prv(ColumnTypeEnum::Snowflake, None, 0);
    assert_eq!(c.is_private, true)
}