use crate::Error::{IncompleteNumber, Overflow};
use std::iter::{from_fn, once};

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const MASK_MSB: u8 = 0b1000_0000;
const BASE: u32 = 128;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|&x| {
            let mut x = x;
            once((x % BASE) as _)
                .chain(from_fn(|| {
                    x /= BASE;
                    Some((x % BASE) as u8 | MASK_MSB).filter(|_| x != 0)
                }))
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut cur: Option<u32> = None;
    let ans: Result<_, _> = bytes
        .iter()
        .filter_map(|&x| {
            cur = cur
                .take()
                .unwrap_or_default()
                .checked_mul(BASE)
                .and_then(|n| n.checked_add((x & (!MASK_MSB)) as _));
            if cur.is_none() {
                Some(Err(Overflow))
            } else if (x & 0b1000_0000) == 0 {
                Some(Ok(cur.take().unwrap()))
            } else {
                None
            }
        })
        .collect();

    if ans.is_err() || cur == None {
        ans
    } else {
        Err(IncompleteNumber)
    }
}
