use std::collections::BTreeMap;
use std::iter::repeat;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&k, v)| v.iter().map(char::to_ascii_lowercase).zip(repeat(k)))
        .collect()
}
