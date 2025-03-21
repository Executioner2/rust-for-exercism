use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
pub struct Flags {
    case_insensitive: bool, // 不区分大小写
    invert_match: bool,     // 反转匹配
    number: bool,           // 输出匹配行数
    exact_match: bool,      // 精确匹配
    least_hit: bool,        // 至少要命中一行才输出文件名
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            case_insensitive: flags.contains(&"-i"),
            invert_match: flags.contains(&"-v"),
            number: flags.contains(&"-n"),
            exact_match: flags.contains(&"-x"),
            least_hit: flags.contains(&"-l"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let mut results = Vec::new();
    let multiple_files = files.len() > 1;

    for &filename in files {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut count = 0;
        let mut exist_match = false;
        while let Ok(n) = reader.read_line(&mut line) {
            if n == 0 {
                break;
            }
            count += 1;

            line = line.trim_end().to_string();
            let new_line = if flags.case_insensitive {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            let is_match = if flags.exact_match {
                new_line == pattern
            } else {
                new_line.contains(&pattern)
            };

            if (is_match && !flags.invert_match) || (!is_match && flags.invert_match) {
                exist_match = true;
                if !flags.least_hit {
                    let filename = if multiple_files {
                        filename.to_string() + ":"
                    } else {
                        "".to_string()
                    };
                    let number = if flags.number {
                        count.to_string() + ":"
                    } else {
                        "".to_string()
                    };
                    results.push(format!("{}{}{}", filename, number, line));
                }
            }
            line.clear();
        }

        if flags.least_hit && exist_match {
            results.push(filename.to_string())
        }
    }

    Ok(results)
}
