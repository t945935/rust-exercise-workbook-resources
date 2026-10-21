use chapter09_iterators::{
    absolute_values, count_containing, first_long_word, format_numbers, normalize_words,
    sum_positive,
};

#[test]
fn sums_only_positive_values_with_wide_result() {
    assert_eq!(sum_positive(&[3, -8, 4, 0]), 7);
    assert_eq!(sum_positive(&[]), 0);
}

#[test]
fn computes_absolute_magnitudes_including_i32_min() {
    assert_eq!(absolute_values(&[-3, 0, 4]), vec![3, 0, 4]);
    assert_eq!(absolute_values(&[i32::MIN]), vec![2_147_483_648]);
}

#[test]
fn trims_lowercases_and_skips_empty_words() {
    assert_eq!(
        normalize_words(&[" Rust ", "  ", "BORROW"]),
        vec![String::from("rust"), String::from("borrow")]
    );
}

#[test]
fn finds_first_word_meeting_scalar_value_threshold() {
    let words = vec![String::from("a"), String::from("好"), String::from("Rust")];
    assert_eq!(first_long_word(&words, 2).map(String::as_str), Some("Rust"));
    assert_eq!(first_long_word(&words, 8), None);
}

#[test]
fn counts_substring_matches_without_changing_words() {
    let words = vec![
        String::from("rust"),
        String::from("trust"),
        String::from("go"),
    ];
    assert_eq!(count_containing(&words, "ust"), 2);
    assert_eq!(count_containing(&words, ""), 3);
    assert_eq!(words.len(), 3);
}

#[test]
fn formats_values_in_order_and_handles_empty_input() {
    assert_eq!(format_numbers(&[4, -2, 0]), "4, -2, 0");
    assert_eq!(format_numbers(&[]), "");
}
