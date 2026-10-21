use chapter05_borrowing::{append_if_missing, first_word, replace_first, split_at_first, utf8_len};

#[test]
fn counts_utf8_bytes_without_taking_ownership() {
    assert_eq!(utf8_len("Rust"), 4);
    assert_eq!(utf8_len("好"), 3);
}

#[test]
fn returns_first_word_as_a_borrowed_slice() {
    assert_eq!(first_word("  Rust book  "), "Rust");
    assert_eq!(first_word("Rust"), "Rust");
    assert_eq!(first_word("   "), "");
}

#[test]
fn splits_at_first_delimiter_and_preserves_remainder() {
    assert_eq!(split_at_first("key:value:tail", ':'), Some(("key", "value:tail")));
    assert_eq!(split_at_first("no delimiter", ':'), None);
    assert_eq!(split_at_first("好：書", '：'), Some(("好", "書")));
}

#[test]
fn appends_suffix_only_when_absent() {
    let mut text = String::from("Rust");
    assert!(append_if_missing(&mut text, "lang"));
    assert_eq!(text, "Rustlang");
    assert!(!append_if_missing(&mut text, "lang"));
    assert_eq!(text, "Rustlang");
}

#[test]
fn replaces_only_first_matching_substring() {
    let mut text = String::from("one one");
    assert!(replace_first(&mut text, "one", "two"));
    assert_eq!(text, "two one");
    assert!(!replace_first(&mut text, "missing", "x"));
    assert!(!replace_first(&mut text, "", "x"));
    let mut unicode = String::from("好，好");
    assert!(replace_first(&mut unicode, "好", "Rust"));
    assert_eq!(unicode, "Rust，好");
}
