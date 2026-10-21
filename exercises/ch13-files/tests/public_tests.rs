use chapter13_files::{
    ParseEntryError, ParseRecordsError, Record, parse_record_line, parse_records, read_utf8,
    relative_child_path,
};
use std::path::Path;

#[test]
fn parses_tab_separated_record_and_reports_field_errors() {
    assert_eq!(
        parse_record_line("Lunch\t1250"),
        Ok(Record {
            description: String::from("Lunch"),
            amount_cents: 1250,
        })
    );
    assert_eq!(parse_record_line("Lunch"), Err(ParseEntryError::MissingTab));
    assert_eq!(
        parse_record_line(" \t1250"),
        Err(ParseEntryError::EmptyDescription)
    );
    assert_eq!(
        parse_record_line("Lunch\t12x"),
        Err(ParseEntryError::InvalidAmount)
    );
}

#[test]
fn parses_lines_and_reports_one_based_error_line() {
    assert_eq!(
        parse_records("Lunch\t1250\n\nBus\t300\n").unwrap(),
        vec![
            Record {
                description: String::from("Lunch"),
                amount_cents: 1250
            },
            Record {
                description: String::from("Bus"),
                amount_cents: 300
            },
        ]
    );
    assert_eq!(
        parse_records("Lunch\t1250\nBus\tbad"),
        Err(ParseRecordsError {
            line_number: 2,
            source: ParseEntryError::InvalidAmount
        })
    );
}

#[test]
fn reads_utf8_fixture_from_a_path() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/records.tsv");
    assert_eq!(read_utf8(&path).unwrap(), "Lunch\t1250\nBus\t300\n");
}

#[test]
fn accepts_relative_children_and_rejects_escaping_paths() {
    let base = Path::new("data");
    assert_eq!(
        relative_child_path(base, Path::new("week/records.tsv")),
        Some(base.join("week/records.tsv"))
    );
    assert_eq!(relative_child_path(base, Path::new("../secret.tsv")), None);
    assert_eq!(
        relative_child_path(base, Path::new("/tmp/secret.tsv")),
        None
    );
}
