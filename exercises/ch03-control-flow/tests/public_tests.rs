use chapter03_control_flow::{
    checked_sum, classify_sign, count_ascii_digits, days_in_month, first_multiple,
};

#[test]
fn classifies_negative_zero_and_positive() {
    assert_eq!(classify_sign(-1), "負數");
    assert_eq!(classify_sign(0), "零");
    assert_eq!(classify_sign(1), "正數");
}

#[test]
fn counts_only_ascii_digits() {
    assert_eq!(count_ascii_digits("a1b23"), 3);
    assert_eq!(count_ascii_digits("１２3"), 1);
    assert_eq!(count_ascii_digits("no digits"), 0);
}

#[test]
fn checked_sum_handles_empty_and_overflow() {
    assert_eq!(checked_sum(&[]), Some(0));
    assert_eq!(checked_sum(&[1, -2, 3]), Some(2));
    assert_eq!(checked_sum(&[i64::MAX, 1]), None);
}

#[test]
fn finds_first_nonzero_divisor_multiple() {
    assert_eq!(first_multiple(&[3, 8, 12, 20], 4), Some(8));
    assert_eq!(first_multiple(&[3, 5], 2), None);
    assert_eq!(first_multiple(&[3, 5], 0), None);
}

#[test]
fn validates_month_and_leap_day() {
    assert_eq!(days_in_month(2024, 2), Some(29));
    assert_eq!(days_in_month(2023, 2), Some(28));
    assert_eq!(days_in_month(2024, 4), Some(30));
    assert_eq!(days_in_month(2024, 13), None);
    assert_eq!(days_in_month(0, 1), None);
}
