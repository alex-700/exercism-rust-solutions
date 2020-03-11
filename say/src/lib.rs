use std::iter::from_fn;

const FROM_0_TO_9: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const FROM_10_TO_19: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const POWERS_OF_THOUSAND: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

fn concat_with(a: &str, b: &str, delimiter: &str) -> String {
    match () {
        _ if a.is_empty() => b.into(),
        _ if b.is_empty() => a.into(),
        _ => format!("{}{}{}", a, delimiter, b),
    }
}

fn encode_two_digits(n: u64) -> String {
    let n = n as usize;
    match n / 10 {
        1 => FROM_10_TO_19[n - 10].into(),
        tens => concat_with(TENS[tens], FROM_0_TO_9[n % 10], "-"),
    }
}

fn encode_three_digits(n: u64) -> String {
    concat_with(
        match n / 100 {
            0 => "".into(),
            h => format!("{} hundred", FROM_0_TO_9[h as usize]),
        }
        .as_str(),
        encode_two_digits(n % 100).as_str(),
        " ",
    )
}

fn encode_num((pow_of_thousand, n): (usize, u64)) -> String {
    if n == 0 {
        "".into()
    } else {
        concat_with(
            encode_three_digits(n).as_str(),
            POWERS_OF_THOUSAND[pow_of_thousand],
            " ",
        )
    }
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".into();
    }

    let mut n = n;
    let mut ans: Vec<_> = from_fn(|| {
        if n == 0 {
            None
        } else {
            let res = n % 1000;
            n /= 1000;
            Some(res)
        }
    })
    .enumerate()
    .map(encode_num)
    .filter(|s| !s.is_empty())
    .collect();
    ans.reverse();
    ans.join(" ")
}
