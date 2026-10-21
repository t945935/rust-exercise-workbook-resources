#![forbid(unsafe_code)]

use std::io::{self, BufRead, Write};

#[derive(Debug)]
pub enum CliError {
    Usage,
    MissingDescription,
    InvalidAmount,
    Io(io::Error),
}

pub fn parse_add_args(_args: &[String]) -> Result<u64, CliError> {
    todo!("Exercise 12-67")
}

pub fn read_description<R: BufRead>(_reader: &mut R) -> io::Result<Option<String>> {
    todo!("Exercise 12-68")
}

pub fn write_entry<W: Write>(
    _writer: &mut W,
    _description: &str,
    _amount_cents: u64,
) -> io::Result<()> {
    todo!("Exercise 12-69")
}

pub fn run_add<R: BufRead, W: Write>(
    _args: &[String],
    _reader: &mut R,
    _writer: &mut W,
) -> Result<(), CliError> {
    todo!("Exercise 12-70")
}
