use std::iter::successors;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

type Result<T> = std::result::Result<T, Error>;

fn check_base(base: u32, err: Error) -> Result<u32> {
    if base > 1 {
        Ok(base)
    } else {
        Err(err)
    }
}

fn check_digit(digit: u32, base: u32) -> Result<u32> {
    if digit < base {
        Ok(digit)
    } else {
        Err(Error::InvalidDigit(digit))
    }
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>> {
    let from_base = check_base(from_base, Error::InvalidInputBase)?;
    let to_base = check_base(to_base, Error::InvalidOutputBase)?;

    let mut num = successors(Some(1), |&x| Some(x * from_base))
        .zip(number.iter().rev())
        .map(|(pow, &d)| check_digit(d, from_base).map(|d| d * pow))
        .sum::<Result<u32>>()?;

    let mut ans = Vec::new();
    while num > 0 {
        ans.push(num % to_base);
        num /= to_base;
    }
    ans.reverse();
    Ok(ans)
}
