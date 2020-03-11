fn normalize(code: &str) -> Result<Vec<u32>, ()> {
    code.chars()
        .filter_map(|c| match c {
            '0'..='9' => Some(Ok(c.to_digit(10).unwrap())),
            ' ' => None,
            _ => Some(Err(())),
        })
        .collect()
}

fn check_sum(v: &[u32]) -> u32 {
    v.iter()
        .rev()
        .zip([0, 1].iter().cycle())
        .map(|(&x, i)| x + i * (x + (x >= 5) as u32))
        .sum()
}

// TODO: rewrite without unnecessary heap allocation
pub fn is_valid(code: &str) -> bool {
    normalize(code)
        .map(|v| v.len() >= 2 && check_sum(&v) % 10 == 0)
        .unwrap_or(false)
}
