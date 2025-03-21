/// Is {candidate} an isogram?
pub fn check(candidate: &str) -> bool {
    candidate.chars()
        .filter(|c| c.is_alphabetic())
        .try_fold(0, |acc, c| {
            let bit = 1 << (c.to_ascii_lowercase() as u8 - b'a');
            if acc & bit == 0 { Ok(acc | bit) } else { Err(()) }
        }).is_ok()
}