// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const DIGIT_HEIGHT: usize = 4;
const DIGIT_WIDTH: usize = 3;

const PATTERNS: [&str; 10] = [
    " _ | ||_|   ", // 0
    "     |  |   ", // 1
    " _  _||_    ", // 2
    " _  _| _|   ", // 3
    "   |_|  |   ", // 4
    " _ |_  _|   ", // 5
    " _ |_ |_|   ", // 6
    " _   |  |   ", // 7
    " _ |_||_|   ", // 8
    " _ |_| _|   ", // 9
];

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();
    let row_count = lines.len();
    let col_count = lines.first().map_or(0, |s| s.len());

    if row_count % DIGIT_HEIGHT != 0 {
        return Err(Error::InvalidRowCount(row_count));
    }
    if col_count % DIGIT_WIDTH != 0 {
        return Err(Error::InvalidColumnCount(col_count));
    }

    let pattern_map: HashMap<_, _> = PATTERNS.iter()
        .enumerate().map(|(i, &s)| (s, i)).collect();

    let result: Vec<String> = lines
        .chunks(DIGIT_HEIGHT)
        .map(|chunk| {
            (0..col_count / DIGIT_WIDTH)
                .map(|col| {
                    let pattern: String = chunk
                        .iter()
                        .flat_map(|&line| line.chars().skip(col * DIGIT_WIDTH).take(DIGIT_WIDTH))
                        .collect();

                    pattern_map
                        .get(pattern.as_str())
                        .map(|&n| n.to_string())
                        .unwrap_or_else(|| "?".to_string())
                })
                .collect()
        })
        .collect();

    Ok(result.join(","))
}
