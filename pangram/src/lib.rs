/// Determine whether a sentence is a pangram.
const ALL_CHARS: u32 = 0x3FFFFFF;
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(0, |bitmap, c| bitmap | (1 << (c.to_ascii_lowercase() as u8 - b'a')))
        == ALL_CHARS
}
