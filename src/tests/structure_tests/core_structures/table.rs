use crate::core_structures::table::Table;

#[test]
fn test_new() {
    let table = Table::new("Test Table", vec![]);
    assert_eq!(table.name, "Test Table".to_string())
}
