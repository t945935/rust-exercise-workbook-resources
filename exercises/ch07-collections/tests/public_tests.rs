use chapter07_collections::{
    append_item, increment_count, lookup_count, prefix_bytes, remove_at, sum_quantities,
    text_metrics,
};
use std::collections::HashMap;

#[test]
fn appends_owned_item_and_reports_new_length() {
    let mut items = vec![String::from("first")];
    assert_eq!(append_item(&mut items, String::from("second")), 2);
    assert_eq!(items, vec![String::from("first"), String::from("second")]);
}

#[test]
fn removes_only_an_existing_index() {
    let mut items = vec![String::from("a"), String::from("b")];
    assert_eq!(remove_at(&mut items, 1), Some(String::from("b")));
    assert_eq!(remove_at(&mut items, 5), None);
    assert_eq!(items, vec![String::from("a")]);
}

#[test]
fn sums_quantities_using_a_wider_accumulator() {
    assert_eq!(sum_quantities(&[1, 2, 3]), 6);
    assert_eq!(sum_quantities(&[u32::MAX, 1]), u128::from(u32::MAX) + 1);
    assert_eq!(sum_quantities(&[]), 0);
}

#[test]
fn looks_up_counts_without_consuming_the_map() {
    let mut counts = HashMap::new();
    counts.insert(String::from("rust"), 3);
    assert_eq!(lookup_count(&counts, "rust"), Some(3));
    assert_eq!(lookup_count(&counts, "go"), None);
    assert_eq!(counts.len(), 1);
}

#[test]
fn increments_map_entry_and_returns_new_count() {
    let mut counts = HashMap::new();
    assert_eq!(increment_count(&mut counts, String::from("rust")), Some(1));
    assert_eq!(increment_count(&mut counts, String::from("rust")), Some(2));
    assert_eq!(increment_count(&mut counts, String::from("go")), Some(1));
    assert_eq!(counts.get("rust"), Some(&2));
    counts.insert(String::from("max"), usize::MAX);
    assert_eq!(increment_count(&mut counts, String::from("max")), None);
    assert_eq!(counts.get("max"), Some(&usize::MAX));
}

#[test]
fn distinguishes_utf8_bytes_from_unicode_scalar_values() {
    assert_eq!(text_metrics("Rust"), (4, 4));
    assert_eq!(text_metrics("好🙂"), (7, 2));
}

#[test]
fn returns_prefix_only_at_valid_utf8_byte_boundaries() {
    assert_eq!(prefix_bytes("好abc", 0), Some(""));
    assert_eq!(prefix_bytes("好abc", 3), Some("好"));
    assert_eq!(prefix_bytes("好abc", 2), None);
    assert_eq!(prefix_bytes("好abc", 99), None);
}
