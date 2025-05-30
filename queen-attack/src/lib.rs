#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(row: i32, col: i32) -> Option<Self> {
        match (row, col) {
            (0..=7, 0..=7) => Some(ChessPosition(row, col)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.0 == other.0.0 || self.0.1 == other.0.1 || (self.0.0 - other.0.0).abs() == (self.0.1 - other.0.1).abs()
    }
}
