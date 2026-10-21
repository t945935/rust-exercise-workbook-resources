use chapter02_values::{
    celsius_to_fahrenheit, format_cents, is_leap_year, parse_bool_flag, safe_ratio,
};

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-10
}

#[test]
fn converts_temperature() {
    assert!(close(celsius_to_fahrenheit(0.0), 32.0));
    assert!(close(celsius_to_fahrenheit(100.0), 212.0));
    assert!(close(celsius_to_fahrenheit(-40.0), -40.0));
}

#[test]
fn applies_gregorian_leap_year_rules() {
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
    assert!(!is_leap_year(0));
}

#[test]
fn ratio_handles_zero_total() {
    assert_eq!(safe_ratio(1, 0), None);
    assert_eq!(safe_ratio(1, 4), Some(0.25));
    assert_eq!(safe_ratio(4, 4), Some(1.0));
}

#[test]
fn formats_signed_cents_without_overflow() {
    assert_eq!(format_cents(0), "0.00");
    assert_eq!(format_cents(1234), "12.34");
    assert_eq!(format_cents(-5), "-0.05");
    assert_eq!(format_cents(i64::MIN), "-92233720368547758.08");
}

#[test]
fn parses_documented_boolean_tokens_case_insensitively() {
    for token in ["true", "TRUE", "yes", "1"] {
        assert_eq!(parse_bool_flag(token), Some(true));
    }
    for token in ["false", "FALSE", "no", "0"] {
        assert_eq!(parse_bool_flag(token), Some(false));
    }
    for token in ["", "maybe", "2"] {
        assert_eq!(parse_bool_flag(token), None);
    }
}
