pub fn number(user_number: &str) -> Option<String> {
    let chars: Vec<char> = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .rev()
        .collect();
    if chars.len() > 11 || chars.len() < 10 || chars[9] <= '1' || chars[6] <= '1' {
        return None;
    }
    if chars.len() == 11 && chars[10] != '1' {
        return None;
    }
    Some(chars.iter().take(10).rev().collect())
}
