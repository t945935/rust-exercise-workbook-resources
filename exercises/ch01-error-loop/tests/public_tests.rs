use chapter01_error_loop::{average_two, classify_exit_code, parse_non_negative};

#[test]
fn classifies_zero_and_every_nonzero_code() {
    assert_eq!(classify_exit_code(0), "成功");
    assert_eq!(classify_exit_code(1), "失敗");
    assert_eq!(classify_exit_code(-1), "失敗");
    assert_eq!(classify_exit_code(i32::MAX), "失敗");
}

#[test]
fn parses_nonnegative_integers_and_rejects_invalid_input() {
    assert_eq!(parse_non_negative(" 42 "), Ok(42));
    assert_eq!(parse_non_negative("0"), Ok(0));
    assert_eq!(parse_non_negative(""), Err("請輸入非負整數"));
    assert_eq!(parse_non_negative("abc"), Err("請輸入非負整數"));
    assert_eq!(parse_non_negative("-2"), Err("請輸入非負整數"));
    assert_eq!(parse_non_negative("4294967296"), Err("請輸入非負整數"));
}

#[test]
fn averages_without_integer_overflow_or_truncation() {
    assert_eq!(average_two(4, 7), 5.5);
    assert_eq!(average_two(i32::MIN, i32::MAX), -0.5);
}
