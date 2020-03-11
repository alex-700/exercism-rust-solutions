/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut cnt_x = 0;
    isbn.chars()
        .filter_map(|c| match c {
            '-' => None,
            'X' => {
                cnt_x += 1;
                Some(Ok(10))
            }
            '0'..='9' => c.to_digit(10).map(Ok),
            _ => Some(Err(())),
        })
        .collect::<Result<Vec<_>, _>>()
        .map_or(false, |vs| {
            vs.len() == 10
                && (cnt_x == 0 || (cnt_x == 1 && *vs.last().unwrap() == 10))
                && 0 == vs
                    .into_iter()
                    .zip((1..=10).rev())
                    .map(|(x, y)| x * y)
                    .sum::<u32>()
                    % 11
        })
}
