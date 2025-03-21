use std::iter::repeat;

macro_rules! ternary {
    ($condition:expr, $t:expr, $f:expr) => {
        if $condition {
            $t
        } else {
            $f
        }
    };
}

/// Return the run-length encoding of {source}.
pub fn encode(source: &str) -> String {
    let mut it = source.chars().peekable();
    let mut count = 0;
    let mut result = String::new();
    while let Some(c) = it.next() {
        count += 1;
        if it.peek() != Some(&c) {
            result.push_str(&format!("{}{}", ternary!(count < 2, "".to_string(), count.to_string()), c));
            count = 0;
        }
    }
    result
}

/// Return the run-length decoding of {source}.
pub fn decode(source: &str) -> String {
    source.chars().fold((String::new(), 0), |(mut result, count), c| {
        if c.is_numeric() {
            (result, count * 10 + c.to_digit(10).unwrap())
        } else {
            result.push_str(&repeat(c).take(1.max(count as usize)).collect::<String>());
            (result, 0)
        }
    }).0
}
