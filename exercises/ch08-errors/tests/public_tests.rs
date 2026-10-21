use chapter08_errors::{
    AmountError, BalanceError, MissingTag, checked_balance, first_nonempty_line, optional_port,
    parse_amount_cents, parse_quantities, require_tag,
};

#[test]
fn finds_first_nonempty_line_as_a_borrowed_slice() {
    let input = "  \n  Rust\nnext";
    assert_eq!(first_nonempty_line(input), Some("Rust"));
    assert_eq!(first_nonempty_line(" \n\t"), None);
}

#[test]
fn parses_optional_port_without_confusing_absence_and_invalid_input() {
    assert_eq!(optional_port(None), Ok(None));
    assert_eq!(optional_port(Some("8080")), Ok(Some(8080)));
    assert!(optional_port(Some("70000")).is_err());
    assert!(optional_port(Some("web")).is_err());
}

#[test]
fn parses_cents_and_classifies_amount_errors() {
    assert_eq!(parse_amount_cents(" 1250 "), Ok(1250));
    assert_eq!(parse_amount_cents(""), Err(AmountError::Empty));
    assert_eq!(parse_amount_cents("-1"), Err(AmountError::Negative));
    assert_eq!(parse_amount_cents("12x"), Err(AmountError::InvalidNumber));
}

#[test]
fn converts_missing_optional_tag_into_a_result_error() {
    assert_eq!(require_tag(Some("food")), Ok("food"));
    assert_eq!(require_tag(None), Err(MissingTag));
}

#[test]
fn checks_balance_overflow_and_negative_result() {
    assert_eq!(checked_balance(100, -25), Ok(75));
    assert_eq!(checked_balance(i64::MAX, 1), Err(BalanceError::Overflow));
    assert_eq!(checked_balance(10, -11), Err(BalanceError::NegativeBalance));
}

#[test]
fn parses_all_quantities_or_returns_the_first_parse_error() {
    assert_eq!(parse_quantities(&["2", "10", "0"]), Ok(vec![2, 10, 0]));
    assert!(parse_quantities(&["2", "bad", "3"]).is_err());
    assert_eq!(parse_quantities(&[]), Ok(vec![]));
}
