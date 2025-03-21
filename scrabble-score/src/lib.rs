const BITMAP: [u64; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
];

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|&c| (0..26).contains(&(c as i16 - b'a' as i16)))
        .map(|c| BITMAP[(c as u8 - b'a') as usize])
        .sum()
}
