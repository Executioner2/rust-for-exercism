pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = Vec::new();
        for i in 0..2.min(row_count as usize) {
            triangle.push(vec![1; i + 1]);
        }
        for i in 2..row_count as usize {
            let mut row = vec![1];
            for j in 1..i {
                row.push(triangle[i - 1][j - 1] + triangle[i - 1][j]);
            }
            row.push(1);
            triangle.push(row);
        }
        Self(triangle)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
