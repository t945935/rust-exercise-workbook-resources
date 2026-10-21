use chapter04_ownership::{
    append_tag, choose_longer, clone_when_needed, consume_and_count, keep_first_drop_second,
    replace_with_default,
};

#[test]
fn consumes_string_and_counts_utf8_bytes() {
    assert_eq!(consume_and_count(String::from("a!")), 2);
    assert_eq!(consume_and_count(String::from("好")), 3);
}

#[test]
fn returns_owned_string_after_appending() {
    assert_eq!(append_tag(String::from("Rust"), "!"), "Rust!");
}

#[test]
fn chooses_longer_by_byte_length_and_prefers_first_on_tie() {
    assert_eq!(
        choose_longer(String::from("rust"), String::from("go")),
        "rust"
    );
    assert_eq!(
        choose_longer(String::from("abc"), String::from("xyz")),
        "abc"
    );
    assert_eq!(choose_longer(String::from("a"), String::from("好")), "好");
}

#[test]
fn returns_first_owned_value_and_drops_second() {
    assert_eq!(
        keep_first_drop_second(String::from("first"), String::from("second")),
        "first"
    );
}

#[test]
fn clones_only_when_both_owned_values_are_needed() {
    assert_eq!(
        clone_when_needed(String::from("log"), true),
        (String::from("log"), Some(String::from("log")))
    );
    assert_eq!(
        clone_when_needed(String::from("log"), false),
        (String::from("log"), None)
    );
}

#[test]
fn moves_current_value_only_on_the_selected_branch() {
    assert_eq!(replace_with_default(String::from("draft"), false), "draft");
    assert_eq!(replace_with_default(String::from("draft"), true), "未命名");
}
