#![forbid(unsafe_code)]

use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub description: String,
    pub amount_cents: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LedgerError {
    MissingTab,
    EmptyDescription,
    InvalidAmount,
    InvalidRecord {
        line_number: usize,
        source: Box<LedgerError>,
    },
    TotalOverflow,
    Io(String),
    Usage,
}

pub fn parse_record_line(_line: &str) -> Result<Record, LedgerError> {
    todo!("Exercise 14-76")
}

pub fn parse_ledger(_contents: &str) -> Result<Vec<Record>, LedgerError> {
    todo!("Exercise 14-77")
}

pub fn total_cents(_records: &[Record]) -> Result<u64, LedgerError> {
    todo!("Exercise 14-78")
}

pub fn run<W: Write>(_args: &[String], _writer: &mut W) -> Result<(), LedgerError> {
    todo!("Exercise 14-79")
}

pub fn read_and_parse(_path: &Path) -> Result<Vec<Record>, LedgerError> {
    todo!("Exercise 14-79")
}

pub fn write_error<W: Write>(writer: &mut W, error: &LedgerError) -> io::Result<()> {
    writeln!(writer, "{error:?}")
}
