#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

type Result<T> = std::result::Result<T, AffineCipherError>;

const ALPHABET_SIZE: i32 = 26;

fn apply(s: &str, f: impl Fn(i32) -> i32) -> String {
    s.chars()
        .filter(|c| !c.is_ascii_whitespace() && !c.is_ascii_punctuation())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let x = (c.to_ascii_lowercase() as u8 - b'a') as i32;
                ((f(x) % ALPHABET_SIZE) as u8 + b'a') as char
            } else {
                c
            }
        })
        .collect()
}

fn split_chunks(s: &str) -> String {
    let mut ans = String::new();
    s.chars().enumerate().for_each(|(i, c)| {
        if i > 0 && i % 5 == 0 {
            ans.push(' ');
        }
        ans.push(c);
    });
    ans
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn rev(x: i32, m: i32) -> i32 {
    if x == 1 {
        1
    } else {
        (1 - rev(m % x, x) * m) / x + m
    }
}

fn check_a(a: i32) -> Result<i32> {
    if gcd(a, ALPHABET_SIZE) != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(a)
    }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String> {
    let a = check_a(a)?;
    Ok(split_chunks(apply(plaintext, |x| a * x + b).as_str()))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String> {
    let a = check_a(a)?;
    let ia = rev(a, ALPHABET_SIZE);
    let b = ALPHABET_SIZE - b % ALPHABET_SIZE;
    Ok(apply(ciphertext, |x| (x + b) * ia))
}
