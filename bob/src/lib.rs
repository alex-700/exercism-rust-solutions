enum MessageType {
    Question,
    Yelling,
    YellingQuestion,
    Nothing,
    Other,
}

fn message_type(message: &str) -> MessageType {
    let s: String = message
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    let yelling = s.chars().any(|c| c.is_ascii_alphabetic())
        && s.chars()
            .filter(char::is_ascii_alphabetic)
            .all(|c| c.is_ascii_uppercase());
    let question = s.ends_with('?');
    use MessageType::*;
    match () {
        _ if question && yelling => YellingQuestion,
        _ if question => Question,
        _ if yelling => Yelling,
        _ if s.is_empty() => Nothing,
        _ => Other,
    }
}

pub fn reply(message: &str) -> &str {
    use MessageType::*;
    match message_type(message) {
        Question => "Sure.",
        Yelling => "Whoa, chill out!",
        YellingQuestion => "Calm down, I know what I'm doing!",
        Nothing => "Fine. Be that way!",
        Other => "Whatever.",
    }
}
