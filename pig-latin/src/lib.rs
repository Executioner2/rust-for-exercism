/// Using the Pig Latin text transformation rules, convert the given input '{input}'
pub fn translate(input: &str) -> String {
    input
        .split(" ")
        .map(do_translate)
        .collect::<Vec<String>>()
        .join(" ")
}

fn split_and_format(input: &str, index: usize, offset: usize) -> String {
    let (part1, part2) = input.split_at(index + offset);
    format!("{}{}ay", part2, part1)
}

fn do_translate(input: &str) -> String {
    let start_with_any =
        |prefixes: &[&str], input: &str| prefixes.iter().any(|&x| input.starts_with(x));

    if start_with_any(&["xr", "yt", "a", "e", "i", "o", "u"], input) {
        return input.to_string() + "ay";
    }

    let split_index = (0..input.len()).find(|&i| {
        start_with_any(&["a", "e", "i", "o", "u", "qu"], &input[i..])
            || (input[i..].starts_with("y") && i > 0)
    });

    if let Some(i) = split_index {
        return if input[i..].starts_with("qu") {
            split_and_format(input, i, 2)
        } else {
            split_and_format(input, i, 0)
        };
    }

    input.to_string()
}
