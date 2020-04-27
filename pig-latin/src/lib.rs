use itertools::Itertools;
use regex::Regex;

pub fn translate(input: &str) -> String {

    let re = Regex::new(r"^(?P<head>[qwrtpsdfghjklzxcvbnmy]*(?:qu|[qwrtpsdfghjklzxcvbnm]))(?P<tail>[a-z]+)$").unwrap();
    input
        .split_whitespace()
        .map(|s| {
            if let Some(c) = re.captures(s) {
                let head = c.name("head").unwrap().as_str();
                if !head.starts_with("xr") && !head.starts_with("yt") {
                    let tail = c.name("tail").unwrap().as_str();
                    return format!("{}{}{}", tail, head, "ay");
                }
            } else if s.starts_with("y") {
                let (_, rest) = s.split_at(1);
                return format!("{}yay", rest);
            }

            format!("{}ay", s)
        })
        .join(" ")
}
