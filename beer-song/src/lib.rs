use itertools::Itertools;

pub fn verse(n: u32) -> String {
    let ns = n.to_string();
    format!(
        "{}{}",
        format!(
            "{0} bottle{2} of beer on the wall, {1} bottle{2} of beer.\n",
            if n == 0 { "No more" } else { &ns },
            if n == 0 { "no more" } else { &ns },
            if n == 1 { "" } else { "s" }
        ),
        format!(
            "{2}, {0} bottle{1} of beer on the wall.\n",
            if n == 0 {
                "99".into()
            } else if n == 1 {
                "no more".into()
            } else {
                (n - 1).to_string()
            },
            if n == 2 { "" } else { "s" },
            if n == 0 {
                "Go to the store and buy some more".into()
            } else {
                format!(
                    "Take {} down and pass it around",
                    if n == 1 { "it" } else { "one" }
                )
            }
        )
    )
}

pub fn sing(start: u32, end: u32) -> String {
    assert!(start >= end);
    (end..=start).rev().map(verse).join("\n")
}
