/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let (mut result, _) = plaintext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .fold((String::new(), 1), |(mut acc, count), c| {
            if c.is_numeric() {
                acc.push(c);
            } else {
                let mut num = c.to_ascii_lowercase() as u8 - b'a';
                num = ((a * num as i32 + b) % 26) as u8;
                acc.push((num + b'a') as char);
            }
            if count % 5 == 0 {
                acc.push(' ');
            }
            (acc, count + 1)
        });
    if result.ends_with(' ') {
        result.pop();
    }
    Ok(result)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let a_1 = mod_inverse(a, 26)?;
    Ok(ciphertext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .fold(String::new(), |mut acc, c| {
            if c.is_numeric() {
                acc.push(c)
            } else {
                let mut num = c.to_ascii_lowercase() as u8 - b'a';
                num = ((num as i32 - b) * a_1).rem_euclid(26) as u8;
                acc.push((num + b'a') as char);
            }
            acc
        }))
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn mod_inverse(a: i32, m: i32) -> Result<i32, AffineCipherError> {
    for x in 1..m {
        if (a * x) % m == 1 {
            return Ok(x);
        }
    }
    Err(AffineCipherError::NotCoprime(a))
}
