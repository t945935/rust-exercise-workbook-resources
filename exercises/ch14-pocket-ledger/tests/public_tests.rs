use pocket_ledger::{LedgerError, Record, parse_ledger, parse_record_line, total_cents};

#[test]
fn parses_record_and_rejects_bad_fields() {
    assert_eq!(
        parse_record_line(" Coffee \t850"),
        Ok(Record {
            description: "Coffee".into(),
            amount_cents: 850
        })
    );
    assert_eq!(parse_record_line("Coffee"), Err(LedgerError::MissingTab));
    assert_eq!(
        parse_record_line(" \t10"),
        Err(LedgerError::EmptyDescription)
    );
    assert_eq!(
        parse_record_line("Coffee\t8.5"),
        Err(LedgerError::InvalidAmount)
    );
}

#[test]
fn preserves_source_line_number_and_accepts_crlf() {
    assert_eq!(
        parse_ledger("Coffee\t850\r\n\r\nBus\tbad"),
        Err(LedgerError::InvalidRecord {
            line_number: 3,
            source: Box::new(LedgerError::InvalidAmount)
        })
    );
    assert_eq!(parse_ledger("Coffee\t850\r\n").unwrap().len(), 1);
    assert!(parse_ledger("").unwrap().is_empty());
}

#[test]
fn adds_integer_cents_and_detects_overflow() {
    let records = vec![
        Record {
            description: "a".into(),
            amount_cents: 800,
        },
        Record {
            description: "b".into(),
            amount_cents: 250,
        },
    ];
    assert_eq!(total_cents(&records), Ok(1050));
    assert_eq!(total_cents(&[]), Ok(0));
    let huge = vec![
        Record {
            description: "a".into(),
            amount_cents: u64::MAX,
        },
        Record {
            description: "b".into(),
            amount_cents: 1,
        },
    ];
    assert_eq!(total_cents(&huge), Err(LedgerError::TotalOverflow));
}

#[test]
fn manifest_fixture_contains_synthetic_records() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sample.tsv");
    let records = parse_ledger(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(records.len(), 2);
    assert_eq!(total_cents(&records), Ok(1100));
}

#[test]
fn parses_file_command_arguments() {
    // Exercise 14-79: complete `run` so commands use an explicit path and writer.
    let args = vec!["check".to_owned(), "tests/fixtures/sample.tsv".to_owned()];
    let mut output = Vec::new();
    assert_eq!(pocket_ledger::run(&args, &mut output), Ok(()));
    assert_eq!(String::from_utf8(output).unwrap(), "OK: 2 records\n");
}

#[test]
fn binary_outputs_total_and_returns_nonzero_on_bad_input() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sample.tsv");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_pocket-ledger"))
        .args(["total", path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "1100\n");
    assert!(output.stderr.is_empty());

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_pocket-ledger"))
        .args(["unknown", path.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}
