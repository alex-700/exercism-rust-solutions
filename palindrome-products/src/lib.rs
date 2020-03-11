use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::{once, repeat};

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    factorizations: HashSet<(u64, u64)>,
    value: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            factorizations: once(if a < b { (a, b) } else { (b, a) }).collect(),
            value: a * b,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factorizations
            .insert(if a < b { (a, b) } else { (b, a) });
    }
}

fn is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    s.chars().rev().zip(s.chars()).all(|(x, y)| x == y)
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut ans = None;
    for (x, y) in (min..=max).flat_map(|x| repeat(x).zip(min..=max)) {
        let prod = x * y;
        if !is_palindrome(prod) {
            continue;
        }
        let p = ans.get_or_insert_with(|| (Palindrome::new(x, y), Palindrome::new(x, y)));
        match p.0.value.cmp(&prod) {
            Ordering::Less => {}
            Ordering::Equal => p.0.insert(x, y),
            Ordering::Greater => p.0 = Palindrome::new(x, y),
        }
        match p.1.value.cmp(&prod) {
            Ordering::Less => p.1 = Palindrome::new(x, y),
            Ordering::Equal => p.1.insert(x, y),
            Ordering::Greater => {}
        }
    }
    ans
}
