/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    fz(plain, |i| if i % 5 == 0 { Some(' ') } else { None })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    fz(cipher, |_| None)
}

fn fz(s: &str, f: fn(i32) -> Option<char>) -> String {
    let (mut res, _) = s.chars().filter(|c| c.is_ascii_alphanumeric()).fold(
        (String::new(), 1),
        |(mut result, acc), c| {
            let c = if c.is_numeric() {
                c
            } else {
                (b'z' - c.to_ascii_lowercase() as u8 + b'a') as char
            };
            result.push(c);
            if let Some(x) = f(acc) {
                result.push(x);
            }
            (result, acc + 1)
        },
    );
    if res.ends_with(' ') {
        res.pop();
    }
    res
}
