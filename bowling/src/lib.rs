use crate::BowlingRound::{Open, Spare, Strike};
use crate::Error::{GameComplete, NotEnoughPinsLeft};
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum BowlingRound {
    Strike,
    Spare(u8),
    Open(u8, u8),
}

#[derive(Default)]
pub struct BowlingGame {
    rounds: Vec<BowlingRound>,
    cur: Option<u8>,
    extra: (Option<u8>, Option<u8>),
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rounds: Vec::with_capacity(10),
            ..Default::default()
        }
    }

    pub fn finished(&self) -> bool {
        self.cur.is_none()
            && self.rounds.len() == 10
            && match (self.rounds[9], self.extra) {
                (Strike, (Some(_), Some(_)))
                | (Spare(_), (Some(_), None))
                | (Open(_, _), (None, None)) => true,
                _ => false,
            }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if self.finished() {
            return Err(GameComplete);
        }
        if pins > 10 {
            return Err(NotEnoughPinsLeft);
        }

        match self.cur.take() {
            None => {
                if self.rounds.len() == 10 {
                    match self.extra {
                        (None, _) => self.extra.0 = Some(pins),
                        (Some(x), None) => match (x % 10).checked_add(pins).filter(|&x| x <= 10) {
                            Some(_) => self.extra.1 = Some(pins),
                            None => return Err(NotEnoughPinsLeft),
                        },
                        _ => unreachable!(),
                    }
                } else if pins == 10 {
                    self.rounds.push(Strike);
                } else {
                    self.cur = Some(pins);
                }
            }
            Some(x) => match x.checked_add(pins).map(|x| x.cmp(&10)) {
                None | Some(Greater) => return Err(NotEnoughPinsLeft),
                Some(Equal) => self.rounds.push(Spare(x)),
                Some(Less) => self.rounds.push(Open(x, pins)),
            },
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.finished() {
            let get_one = |i, second_in_strike: bool| match self.rounds.get(i).cloned() {
                Some(Strike) => 10,
                Some(Spare(x)) | Some(Open(x, _)) => x,
                None => {
                    if second_in_strike {
                        self.extra.0.unwrap_or(0)
                    } else {
                        0
                    }
                }
            };
            let get_two = |i| match self.rounds.get(i) {
                Some(Strike) => 10 + get_one(i + 1, true),
                Some(Spare(_)) => 10,
                Some(Open(x, y)) => x + y,
                None => 0,
            };
            Some(
                self.rounds
                    .iter()
                    .enumerate()
                    .map(|(i, &x)| {
                        u16::from(match x {
                            Strike => 10 + get_two(i + 1),
                            Spare(_) => 10 + get_one(i + 1, false),
                            Open(a, b) => a + b,
                        })
                    })
                    .chain(self.extra.0.map(u16::from))
                    .chain(self.extra.1.map(u16::from))
                    .sum(),
            )
        } else {
            None
        }
    }
}
