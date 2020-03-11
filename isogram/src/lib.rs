use itertools::Itertools;

pub fn check(candidate: &str) -> bool {
    let r = candidate
        .chars()
        .filter(|&c| c != ' ' && c != '-')
        .map(|c| c.to_ascii_lowercase());
    r.clone().count() == r.unique().count()
}
