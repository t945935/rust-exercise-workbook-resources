#![forbid(unsafe_code)]

use std::fmt::Display;

pub fn first_or<T: Clone>(_items: &[T], _fallback: T) -> T {
    todo!("Exercise 10-56")
}

pub fn is_nondecreasing<T: Ord>(_items: &[T]) -> bool {
    todo!("Exercise 10-57")
}

pub fn count_matches<T: PartialEq>(_items: &[T], _target: &T) -> usize {
    todo!("Exercise 10-58")
}

pub fn format_pair<L: Display, R: Display>(_left: &L, _right: &R) -> String {
    todo!("Exercise 10-59")
}

pub trait Label {
    fn label(&self) -> &str;
}

pub fn label_matches<T: Label>(_item: &T, _expected: &str) -> bool {
    todo!("Exercise 10-60")
}

pub fn longer<'a>(_left: &'a str, _right: &'a str) -> &'a str {
    todo!("Exercise 10-61")
}
