use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

const LETTERS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];

fn repeat_char(c: char, x: u32) -> String {
    std::iter::repeat(c).take(x as usize).collect()
}

fn get((i, digit): (usize, u32)) -> String {
    let i = 2 * i;
    let c = LETTERS[i];

    match digit {
        0..=3 => repeat_char(c, digit),
        4 => format!("{}{}", c, LETTERS[i + 1]),
        5..=8 => format!("{}{}", LETTERS[i + 1], repeat_char(c, digit - 5)),
        9 => format!("{}{}", c, LETTERS[i + 2]),
        _ => unreachable!(),
    }
}

fn digits(n: u32) -> Vec<(usize, u32)> {
    let mut n = n;
    let mut v: Vec<_> = std::iter::from_fn(|| {
        let res = Some(n % 10).filter(|_| n != 0);
        n /= 10;
        res
    })
    .enumerate()
    .collect();
    v.reverse();
    v
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            digits(self.0).into_iter().map(get).collect::<String>()
        )
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}
