/// What are the series of length {len} in string {digits:?}
pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();
    let end = digits.len() as i32 - len as i32;
    for i in 0..=end {
        let i = i as usize;
        result.push(digits[i..i + len].to_string())
    }

    result
}
