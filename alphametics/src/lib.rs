use itertools::Itertools;
use std::collections::HashMap;
use std::hash::BuildHasher;

fn word_to_int<S: BuildHasher>(word: &str, h: &HashMap<char, u8, S>) -> Option<u64> {
    if word.len() > 1 && h[&word.chars().next().unwrap()] == 0 {
        None
    } else {
        Some(word.chars().fold(0, |a, c| a * 10 + u64::from(h[&c])))
    }
}

fn words_to_int<S: BuildHasher>(words: &[&str], h: &HashMap<char, u8, S>) -> Option<u64> {
    words.iter().map(|&s| word_to_int(s, h)).sum()
}

struct Equation<'a> {
    left: Vec<&'a str>,
    right: Vec<&'a str>,

    letters: Vec<char>,
}

impl<'a> Equation<'a> {
    fn new(s: &'a str) -> Self {
        let v = s.split(" == ").collect_vec();
        let left = v[0].split(" + ").collect_vec();
        let right = v[1].split(" + ").collect_vec();
        let letters = left
            .iter()
            .chain(right.iter())
            .flat_map(|s| s.chars())
            .unique()
            .collect();

        Equation {
            left,
            right,
            letters,
        }
    }

    fn check<S: BuildHasher>(&self, h: &HashMap<char, u8, S>) -> bool {
        let a = words_to_int(&self.left, h);
        let b = words_to_int(&self.right, h);
        a == b && a.is_some()
    }

    fn solve(&self) -> Option<HashMap<char, u8>> {
        (0_u8..10)
            .permutations(self.letters.len())
            .find_map(|digits| {
                Some(
                    self.letters
                        .iter()
                        .cloned()
                        .zip(digits.into_iter())
                        .collect(),
                )
                .filter(|h| self.check(h))
            })
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    Equation::new(input).solve()
}
