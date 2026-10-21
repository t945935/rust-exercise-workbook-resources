#![forbid(unsafe_code)]

use std::num::ParseIntError;

pub fn first_nonempty_line(_input: &str) -> Option<&str> {
    todo!("Exercise 08-44")
}

pub fn optional_port(_input: Option<&str>) -> Result<Option<u16>, ParseIntError> {
    todo!("Exercise 08-45")
}

#[derive(Debug, PartialEq, Eq)]
pub enum AmountError {
    Empty,
    Negative,
    InvalidNumber,
}

pub fn parse_amount_cents(_input: &str) -> Result<u64, AmountError> {
    todo!("Exercise 08-46")
}

#[derive(Debug, PartialEq, Eq)]
pub struct MissingTag;

pub fn require_tag(_tag: Option<&str>) -> Result<&str, MissingTag> {
    todo!("Exercise 08-47")
}

#[derive(Debug, PartialEq, Eq)]
pub enum BalanceError {
    Overflow,
    NegativeBalance,
}

pub fn checked_balance(_current: i64, _delta: i64) -> Result<i64, BalanceError> {
    todo!("Exercise 08-48")
}

pub fn parse_quantities(_inputs: &[&str]) -> Result<Vec<u32>, ParseIntError> {
    todo!("Exercise 08-49")
}
