use itertools::{repeat_n, Itertools};

pub fn encode(source: &str) -> String {
    source
        .chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, group)| match group.count() {
            1 => format!("{}", c),
            n => format!("{}{}", n, c),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .batching(|it| {
            let cnt = it
                .take_while_ref(|c| c.is_ascii_digit())
                .fold(None, |a, b| {
                    Some(a.unwrap_or(0) * 10 + b.to_digit(10).unwrap())
                })
                .unwrap_or(1);
            it.next().map(|x| repeat_n(x, cnt as usize))
        })
        .flatten()
        .collect()
}
