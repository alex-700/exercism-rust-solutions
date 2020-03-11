#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

fn validate(x: i32) -> Option<i32> {
    Some(x).filter(|x| (0..8).contains(x))
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        validate(rank).and_then(|x| validate(file).map(|y| ChessPosition(x, y)))
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let a = &self.position;
        let b = &other.position;

        a.0 == b.0 || a.1 == b.1 || a.0 - a.1 == b.0 - b.1 || a.0 + a.1 == b.0 + b.1
    }
}
