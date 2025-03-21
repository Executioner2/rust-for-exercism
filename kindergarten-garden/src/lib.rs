pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let first_char = student.chars().next().unwrap().to_ascii_uppercase();
    let row = ((first_char as u8 - b'A') * 2) as usize;
    let mut res = vec![];

    let f = |c: char| -> &str {
        match c {
            'C' => "clover",
            'G' => "grass",
            'R' => "radishes",
            'V' => "violets",
            _ => "unknown",
        }
    };

    for line in diagram.lines() {
        res.push(f(line.chars().nth(row).unwrap()));
        res.push(f(line.chars().nth(row + 1).unwrap()));
    }

    res
}
