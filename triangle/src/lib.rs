pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T: Copy + std::ops::Add<Output = T> + PartialEq + PartialOrd> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let mut sides = sides;
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if sides[0] + sides[1] > sides[2] {
            Some(Self { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b && b == c
    }

    pub fn is_scalene(&self) -> bool {
        let [a, b, c] = self.sides;
        a != b && b != c
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b || b == c
    }
}
