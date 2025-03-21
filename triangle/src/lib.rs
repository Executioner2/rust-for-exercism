use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T> where T: PartialOrd + Copy + Add<Output = T> {
    pub fn build(mut sides: [T; 3]) -> Option<Self> {
        sides.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        if sides[0] + sides[1] <= sides[2] { return None; }
        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !(self.is_equilateral() || self.is_isosceles())
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2]
    }
}
