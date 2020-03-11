use std::char::from_digit;
use std::ops::RangeInclusive;

fn count_of_mines(it: impl Iterator<Item = char>) -> char {
    match it.filter(|&c| c == '*').count() {
        0 => ' ',
        c => from_digit(c as u32, 10).unwrap(),
    }
}

fn neighbors(n: usize, max: usize) -> RangeInclusive<usize> {
    n.saturating_sub(1)..=n.checked_add(1).filter(|&n| n < max).unwrap_or(max - 1)
}

fn digit(minefield: &[&str], r: usize, c: usize) -> char {
    count_of_mines(
        minefield[neighbors(r, minefield.len())]
            .iter()
            .flat_map(|s| s[neighbors(c, minefield[0].len())].chars()),
    )
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| if c == ' ' { digit(minefield, i, j) } else { c })
                .collect()
        })
        .collect()
}
