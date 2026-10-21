use chapter11_modules::amounts::is_valid_cents;
use chapter11_modules::records::{Entry, total_cents};

#[test]
fn exposes_module_function_through_public_path() {
    assert!(is_valid_cents(0));
    assert!(is_valid_cents(1250));
    assert!(!is_valid_cents(-1));
}

#[test]
fn constructs_entries_with_private_fields_and_public_accessors() {
    let entry = Entry::new("  lunch  ", 1250).unwrap();
    assert_eq!(entry.description(), "lunch");
    assert_eq!(entry.amount_cents(), 1250);
    assert!(Entry::new(" \t ", 100).is_none());
    assert!(Entry::new("refund", -1).is_none());
}

#[test]
fn totals_entries_without_exposing_their_fields() {
    let entries = [
        Entry::new("bus", 300).unwrap(),
        Entry::new("meal", 1250).unwrap(),
    ];
    assert_eq!(total_cents(&entries), 1550);
    assert_eq!(total_cents(&[]), 0);
}
