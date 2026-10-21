use chapter10_generics::{
    Label, count_matches, first_or, format_pair, is_nondecreasing, label_matches, longer,
};

struct ItemLabel(String);

impl Label for ItemLabel {
    fn label(&self) -> &str {
        &self.0
    }
}

#[test]
fn returns_cloned_first_item_or_owned_fallback() {
    assert_eq!(
        first_or(&[String::from("first")], String::from("fallback")),
        "first"
    );
    assert_eq!(
        first_or::<String>(&[], String::from("fallback")),
        "fallback"
    );
}

#[test]
fn checks_order_for_generic_ordered_items() {
    assert!(is_nondecreasing(&[1, 1, 3]));
    assert!(!is_nondecreasing(&[2, 1]));
    assert!(is_nondecreasing::<i32>(&[]));
}

#[test]
fn counts_equal_items_without_requiring_clone() {
    let words = [
        String::from("rust"),
        String::from("go"),
        String::from("rust"),
    ];
    assert_eq!(count_matches(&words, &String::from("rust")), 2);
}

#[test]
fn formats_values_of_different_display_types() {
    assert_eq!(format_pair(&4, &"items"), "4: items");
}

#[test]
fn uses_a_trait_bound_to_check_a_label() {
    let item = ItemLabel(String::from("food"));
    assert!(label_matches(&item, "food"));
    assert!(!label_matches(&item, "travel"));
}

#[test]
fn returns_the_longer_slice_and_prefers_left_on_ties() {
    assert_eq!(longer("Rust", "Go"), "Rust");
    assert_eq!(longer("same", "size"), "same");
}
