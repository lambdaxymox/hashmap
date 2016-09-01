use std::collections::hash_map::HashMap;

#[test]
fn test_initialize_empty_hash_table() {
    let hash_table: HashMap<usize, usize> = hashmap!();

    assert_eq!(hash_table.len(), 0);
}

#[test]
fn test_initialize_single_element_hash_table() {
    let hash_table = hashmap!(1 => 'a');

    assert_eq!(hash_table.len(), 1);
}

#[test]
fn test_initialize_multiple_element_hash_table() {
    let hash_table = hashmap!(1 => 'a', 2 => 'b', 3 => 'c');

    assert_eq!(hash_table.len(), 3);
}

#[test]
fn test_macro_generate_hashmap_should_equal_manually_generated_one() {
    let hashmap1 = hashmap!(1 => 'a', 2 => 'b', 3 => 'c', 4 => 'd');

    let mut hashmap2 = HashMap::new();
    hashmap2.insert(1, 'a');
    hashmap2.insert(2, 'b');
    hashmap2.insert(3, 'c');
    hashmap2.insert(4, 'd');

    assert_eq!(hashmap1, hashmap2);
}
