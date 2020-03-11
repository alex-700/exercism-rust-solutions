pub fn number(user_number: &str) -> Option<String> {
    user_number
        .chars()
        .filter(|c| !c.is_ascii_punctuation() && !c.is_ascii_whitespace())
        .map(|c| Some(c).filter(|c| c.is_ascii_digit()))
        .collect::<Option<Vec<_>>>()
        .map(|mut v| {
            if v.len() == 11 && v[0] == '1' {
                v.remove(0);
            }
            v
        })
        .filter(|v| v.len() == 10 && !"01".contains(v[0]) && !"01".contains(v[3]))
        .map(|v| v.into_iter().collect())
}
