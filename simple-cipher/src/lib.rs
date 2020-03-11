use rand::{thread_rng, Rng};
use std::iter::repeat_with;

fn ascii_op(op: impl Fn(u8, u8) -> u8) -> impl Fn(char, char) -> Option<char> {
    move |a, b| {
        if a.is_ascii_lowercase() && b.is_ascii_lowercase() {
            Some((op(a as u8 - b'a', b as u8 - b'a') % 26 + b'a') as char)
        } else {
            None
        }
    }
}

fn calculate(key: &str, s: &str, op: impl Fn(u8, u8) -> u8) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    let op = ascii_op(op);
    s.chars()
        .zip(key.chars().cycle())
        .map(|(x, y)| op(x, y))
        .collect()
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    calculate(key, s, |x, y| x + y)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    calculate(key, s, |x, y| x + 26 - y)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = repeat_with(|| thread_rng().gen_range(b'a', b'z' + 1) as char)
        .take(100)
        .collect();
    let cipher = encode(&key, s).unwrap();
    (key, cipher)
}
