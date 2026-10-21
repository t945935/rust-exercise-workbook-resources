#![forbid(unsafe_code)]

use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub description: String,
    pub amount_cents: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseEntryError {
    MissingTab,
    EmptyDescription,
    InvalidAmount,
}

pub fn parse_record_line(_line: &str) -> Result<Record, ParseEntryError> {
    todo!("Exercise 13-72")
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRecordsError {
    pub line_number: usize,
    pub source: ParseEntryError,
}

pub fn parse_records(_contents: &str) -> Result<Vec<Record>, ParseRecordsError> {
    todo!("Exercise 13-73")
}

pub fn read_utf8(_path: &Path) -> io::Result<String> {
    todo!("Exercise 13-74")
}

pub fn relative_child_path(_base: &Path, _name: &Path) -> Option<PathBuf> {
    todo!("Exercise 13-75")
}
