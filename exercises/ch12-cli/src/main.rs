#![forbid(unsafe_code)]

use std::io;

fn main() {
    let _args: Vec<String> = std::env::args().skip(1).collect();
    let _stdin = io::stdin();
    let _stdout = io::stdout();
    todo!("Exercise 12-71")
}
