use itertools::Itertools;
use regex::Regex;

pub fn translate(input: &str) -> String {
    let re = Regex::new(r"^(?P<head>[qwrtpsdfghjklzxcvbnm]+(?:qu)?)(?P<tail>[a-z]+)$").unwrap();
    input
        .split_whitespace()
        .map(|s| {
            if let Some(c) = re.captures(s) {
                let head = c.name("head").unwrap().as_str();
                let tail = c.name("tail").unwrap().as_str();
                format!("{}{}{}", tail, head, "ay")
            } else {
                format!("{}ay", s)
            }
        })
        .join(" ")
}
