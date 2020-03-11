use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    Regex::new("[A-Z][a-z']+|[a-z]+|[A-Z]+")
        .unwrap()
        .find_iter(phrase)
        .map(|s| s.as_str().chars().next().unwrap().to_ascii_uppercase())
        .collect()
}
