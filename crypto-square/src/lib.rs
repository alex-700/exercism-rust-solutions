use itertools::Itertools;
use num_integer::Roots;

fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn size_of_rectangle(n: usize) -> (usize, usize) {
    if n == 0 {
        (0, 0)
    } else {
        let r = n.sqrt();
        (r, (n + r - 1) / r)
    }
}

pub fn encrypt(input: &str) -> String {
    let s = normalize(input);
    let (r, c) = size_of_rectangle(s.len());
    let s: String = s
        .chars()
        .chain(std::iter::repeat(' '))
        .take(r * c)
        .collect();

    (0..c)
        .map(|i| s.chars().skip(i).step_by(c).collect::<String>())
        .join(" ")
}
