use regex::Regex;
use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let words = words.to_ascii_lowercase();
    let re = Regex::new(r"[a-z]+('[a-z]+)?|\d+").unwrap();
    let mut ans = HashMap::new();
    for x in re.find_iter(&words) {
        *ans.entry(x.as_str().to_string()).or_default() += 1;
    }
    ans
}
