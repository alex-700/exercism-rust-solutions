use std::iter::{once, successors};

pub struct PascalsTriangle {
    row_count: usize,
}

fn next(v: &[u32]) -> Vec<u32> {
    once(1)
        .chain(v.windows(2).map(|x| x.iter().sum()))
        .chain(once(1))
        .collect()
}

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        successors(Some(vec![1]), |x| Some(next(x)))
            .take(self.row_count)
            .collect()
    }
}
