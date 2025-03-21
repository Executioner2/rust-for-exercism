#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

/// largest series product of a span of {span} digits in {string_digits}
pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if string_digits.is_empty() {
        return Ok(1);
    }

    let mut prod = 1u64;
    let mut max = u64::MIN;
    let mut k = 0;
    let bytes = string_digits.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        let val = match b {
            b'0'..=b'9' => *b - b'0',
            _ => return Err(Error::InvalidDigit(*b as char)),
        };
        
        k += 1;
        if val == 0 {
            k = 0;
            prod = 1;
        } else if k > span {
            prod = prod / 1.max(bytes[i - span] - b'0') as u64 * val as u64;
        } else {
            prod *= val as u64;
        };
        if k >= span { max = max.max(prod); }
    }

    Ok(max)
}
