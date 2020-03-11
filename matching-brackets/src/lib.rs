pub fn brackets_are_balanced(string: &str) -> bool {
    let mut s = Vec::new();
    fn open_to_closed(c: char) -> Option<char> {
        match c {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            _ => None,
        }
    }
    string.chars().all(|c| match c {
        '(' | '[' | '{' => {
            s.push(c);
            true
        }
        ')' | ']' | '}' => s.pop().and_then(open_to_closed) == Some(c),
        _ => true,
    }) && s.is_empty()
}
