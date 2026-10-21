use chapter12_cli::{CliError, parse_add_args, read_description, run_add, write_entry};
use std::io::Cursor;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn parses_add_command_and_nonnegative_integer_cents() {
    let valid = vec![String::from("add"), String::from("1250")];
    assert_eq!(parse_add_args(&valid).unwrap(), 1250);
    let invalid = vec![String::from("add"), String::from("12x")];
    assert!(matches!(
        parse_add_args(&invalid),
        Err(CliError::InvalidAmount)
    ));
    assert!(matches!(parse_add_args(&[]), Err(CliError::Usage)));
}

#[test]
fn reads_one_line_without_line_ending_and_detects_eof() {
    let mut reader = Cursor::new(b"Lunch\r\n".to_vec());
    assert_eq!(
        read_description(&mut reader).unwrap(),
        Some(String::from("Lunch"))
    );
    assert_eq!(read_description(&mut reader).unwrap(), None);
}

#[test]
fn writes_a_tab_separated_record_line() {
    let mut output = Vec::new();
    write_entry(&mut output, "Lunch", 1250).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Lunch\t1250\n");
}

#[test]
fn runs_add_with_injected_streams() {
    let args = vec![String::from("add"), String::from("1250")];
    let mut input = Cursor::new(b"Lunch\n".to_vec());
    let mut output = Vec::new();
    run_add(&args, &mut input, &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Lunch\t1250\n");
}

#[test]
fn executable_reads_stdin_and_writes_stdout() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_chapter12-cli"))
        .args(["add", "1250"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"Lunch\n").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Lunch\t1250\n");
}

#[test]
fn executable_returns_nonzero_for_invalid_amount() {
    let output = Command::new(env!("CARGO_BIN_EXE_chapter12-cli"))
        .args(["add", "not-a-number"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("InvalidAmount")
    );
}
