use std::collections::HashSet;
use std::iter::successors;

pub fn collatz(n: u64) -> Option<usize> {
    let mut mem = HashSet::new();
    successors(Some(n), |&i| {
        if mem.insert(i) {
            Some(if i % 2 == 0 { i / 2 } else { 3 * i + 1 })
        } else {
            None
        }
    })
    .position(|x| x == 1)
}
