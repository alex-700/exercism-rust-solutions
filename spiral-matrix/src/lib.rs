use std::ops::{AddAssign, SubAssign};

struct CellIdx(i32, i32);

fn check_index(idx: i32, size: usize) -> bool {
    0 <= idx && idx < size as i32
}

impl CellIdx {
    fn check(&self, matrix: &[Vec<u32>]) -> bool {
        check_index(self.0, matrix.len())
            && check_index(self.1, matrix[0].len())
            && matrix[self.0 as usize][self.1 as usize] == 0
    }
}

#[derive(Copy, Clone)]
struct Shift(i32, i32);

impl Shift {
    fn next(self) -> Self {
        if self.0 == 0 {
            Shift(self.1, self.0)
        } else {
            Shift(self.1, -self.0)
        }
    }
}

impl AddAssign<Shift> for CellIdx {
    fn add_assign(&mut self, rhs: Shift) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign<Shift> for CellIdx {
    fn sub_assign(&mut self, rhs: Shift) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let mut idx = CellIdx(0, 0);
    let mut shift = Shift(0, 1);
    for el in 1..=size * size {
        matrix[idx.0 as usize][idx.1 as usize] = el;
        idx += shift;
        if !idx.check(&matrix) {
            idx -= shift;
            shift = shift.next();
            idx += shift;
        }
    }
    matrix
}
