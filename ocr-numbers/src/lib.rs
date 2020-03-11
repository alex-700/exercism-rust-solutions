use itertools::Itertools;
use lazy_static::lazy_static;

use crate::Error::{InvalidColumnCount, InvalidRowCount};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

type Result<T> = std::result::Result<T, Error>;

fn check_row_count(n: usize) -> Result<usize> {
    if n % 4 == 0 && n > 0 {
        Ok(n)
    } else {
        Err(InvalidRowCount(n))
    }
}

fn check_column_count(n: usize) -> Result<usize> {
    if n % 3 == 0 && n > 0 {
        Ok(n)
    } else {
        Err(InvalidColumnCount(n))
    }
}

fn check_sizes(input: &str) -> Result<Vec<&str>> {
    let v: Vec<_> = input.lines().collect();
    let _rows = check_row_count(v.len())?;
    let cols = check_column_count(v[0].len())?;
    assert!(v.iter().all(|s| s.len() == cols));
    Ok(v)
}

const PATTERNS: [&str; 4] = [
    " _     _  _     _  _  _  _  _ ",
    "| |  | _| _||_||_ |_   ||_||_|",
    "|_|  ||_  _|  | _||_|  ||_| _|",
    "                              ",
];

lazy_static! {
    static ref PATTERN_TO_DIGIT: HashMap<String, char> = (0..=9)
        .map(|n| (
            PATTERNS.iter().map(|x| &x[3 * n..3 * (n + 1)]).collect(),
            (b'0' + n as u8) as char
        ))
        .collect();
}

fn convert_digit(input: &[&str]) -> char {
    PATTERN_TO_DIGIT
        .get(&input.concat())
        .cloned()
        .unwrap_or('?')
}

fn convert_number(input: &[&str]) -> String {
    (0..input[0].len())
        .step_by(3)
        .map(|x| {
            convert_digit(&[
                &input[0][x..x + 3],
                &input[1][x..x + 3],
                &input[2][x..x + 3],
                &input[3][x..x + 3],
            ])
        })
        .collect()
}

pub fn convert(input: &str) -> Result<String> {
    Ok(check_sizes(input)?.chunks(4).map(convert_number).join(","))
}
