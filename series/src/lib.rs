pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .map(|x| String::from_utf8_lossy(x).into())
        .collect()
}
