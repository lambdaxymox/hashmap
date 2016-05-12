use std::collections::hash_map::HashMap;

#[test]
fn test_initialize_empty_hash_table() {
    let hash_table: HashMap<usize, usize> = hash_map!();

    assert_eq!(hash_table.len(), 0);
}

#[test]
fn test_initialize_single_element_hash_table() {
    let hash_table = hash_map! { 1 => 'a'};

    assert_eq!(hash_table.len(), 1);
}

#[test]
fn test_initialize_multiple_element_hash_table() {
    let hash_table = hash_map! { 1 => 'a', 2 => 'b', 3 => 'c'};

    assert_eq!(hash_table.len(), 3);
}