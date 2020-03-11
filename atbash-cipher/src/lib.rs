use itertools::Itertools;

fn transform_ascii_alphabetic(c: char) -> char {
    (b'a' + 25 - (c.to_ascii_lowercase() as u8 - b'a')) as char
}

fn transform_str(s: &str) -> String {
    s.chars()
        .filter_map(|x| match x {
            _ if x.is_ascii_alphabetic() => Some(transform_ascii_alphabetic(x)),
            _ if x.is_ascii_whitespace() => None,
            _ if x.is_ascii_punctuation() => None,
            _ => Some(x),
        })
        .collect()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    transform_str(plain)
        .chars()
        .chunks(5)
        .into_iter()
        .map(|c| c.collect::<String>())
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    transform_str(cipher)
}
