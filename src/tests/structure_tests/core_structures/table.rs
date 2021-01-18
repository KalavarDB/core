use crate::core_structures::table::Table;

#[test]
fn test_new() {
    let table = Table::new("Test Table", vec![]);
    assert_eq!(table.name, "Test Table".to_string())
}

#[test]
fn test_new_default_table() {
    let table = Table::new("Test Table", vec![]);
    assert!(table.columns.contains_key("__IDENTIFIER__") && table.columns.get("__IDENTIFIER__").unwrap().is_private);
}
