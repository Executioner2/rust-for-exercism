pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                return c;
            }
            let new_c = c.to_ascii_lowercase();
            let mut new_c = (new_c as u8 + key - b'a') % 26 + b'a';
            if c.is_ascii_uppercase() {
                new_c -= 32;
            }
            new_c as char
        })
        .collect::<String>()
}
