pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".into(); digits.len() + 1];
    }
    digits
        .as_bytes()
        .windows(len)
        .map(|x| String::from_utf8_lossy(x).into())
        .collect()
}
