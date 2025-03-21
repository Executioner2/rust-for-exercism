/// the number of grains on a given square
pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("square must be between 1 and 64")
    }
    (2u64).pow(s - 1)
}

/// the total number of grains on the chessboard
pub fn total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + square(x))
}
