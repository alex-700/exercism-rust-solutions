use std::cmp::Ordering;
use Comparison::*;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist_impl<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    small.is_empty() || big.windows(small.len()).any(|x| x == small)
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match first.len().cmp(&second.len()) {
        Ordering::Less if sublist_impl(first, second) => Sublist,
        Ordering::Equal if first == second => Equal,
        Ordering::Greater if sublist_impl(second, first) => Superlist,
        _ => Unequal,
    }
}
