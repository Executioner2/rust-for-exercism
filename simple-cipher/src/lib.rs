pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    let res = key.chars().cycle()
        .zip(s.chars())
        .map(|(k, s)| (k as u8 - b'a') + (s as u8 - b'a'))
        .map(|n| (n % 26 + b'a') as char)
        .collect::<String>();
    Some(res)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    let res = key.chars().cycle()
        .zip(s.chars())
        .map(|(k, s)|(s as u8 - b'a') as i32 - (k as u8 - b'a') as i32 + 26)
        .map(|n| ((n % 26) as u8 + b'a') as char)
        .collect::<String>();
    Some(res)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = (0..100)
       .map(|_| rand::random::<u8>() % 26)
       .map(|n| (n + b'a') as char)
       .collect();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}
