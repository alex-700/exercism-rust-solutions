pub fn rotate(input: &str, key: i8) -> String {
    let key = (((key % 26) + 26) % 26) as u8;
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => (b'a' + ((c as u8 - b'a') + key) % 26) as char,
            'A'..='Z' => (b'A' + ((c as u8 - b'A') + key) % 26) as char,
            _ => c,
        })
        .collect()
}
