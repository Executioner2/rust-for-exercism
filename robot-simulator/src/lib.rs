// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: usize,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn parse_direction(d: &Direction) -> (i32, i32) {
    match d {
        Direction::North => (0, 1),
        Direction::East => (1, 0),
        Direction::South => (0, -1),
        Direction::West => (-1, 0),
    }
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        let d = DIRECTIONS.iter().position(|dir| *dir == d).unwrap();
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.d = (self.d + 1) & 3;
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.d = ((self.d as isize - 1) & 3) as usize;
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        let (x, y) = parse_direction(&DIRECTIONS[self.d]);
        self.x += x;
        self.y += y;
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            self = match c {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                'A' => self.advance(),
                _ => self,
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &DIRECTIONS[self.d]
    }
}
