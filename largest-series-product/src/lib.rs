use crate::Error::{InvalidDigit, SpanTooLong};

#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

type Result<T> = std::result::Result<T, Error>;

fn check_span(span: usize, max_span: usize) -> Result<usize> {
    if span <= max_span {
        Ok(span)
    } else {
        Err(SpanTooLong)
    }
}

fn get_digit(string_digits: &str, idx: usize) -> Result<u64> {
    let c = string_digits.as_bytes()[idx] as char;
    c.to_digit(10).map(u64::from).ok_or(InvalidDigit(c))
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64> {
    let span = check_span(span, string_digits.len())?;
    let mut head = 0;
    let mut cur = 1;
    let mut ans = if span == 0 { 1 } else { 0 };
    for tail in 1..=string_digits.len() {
        let d = get_digit(string_digits, tail - 1)?;
        if d == 0 {
            head = tail;
            cur = 1;
        } else {
            cur *= d;
            if head + span == tail {
                ans = std::cmp::max(ans, cur);
                cur /= get_digit(string_digits, head).unwrap();
                head += 1;
            }
        }
    }
    Ok(ans)
}
