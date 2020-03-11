use crate::Direction::{East, North, South, West};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

fn sum(x: (i32, i32), y: (i32, i32)) -> (i32, i32) {
    (x.0 + y.0, x.1 + y.1)
}

fn shift(d: Direction) -> (i32, i32) {
    match d {
        North => (0, 1),
        East => (1, 0),
        South => (0, -1),
        West => (-1, 0),
    }
}

pub struct Robot {
    pos: (i32, i32),
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { pos: (x, y), d }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            d: match self.d {
                North => East,
                East => South,
                South => West,
                West => North,
            },
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            d: match self.d {
                North => West,
                East => North,
                South => East,
                West => South,
            },
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Robot {
            pos: sum(self.pos, shift(self.d)),
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'A' => robot.advance(),
            _ => unreachable!(),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
