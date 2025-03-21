/// Annotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let angle = [-1, 0, 1, 0, -1, -1, 1, 1, -1];
    let height = minefield.len() as i32;
    (0..height)
        .map(|i| {
            let width = minefield[i as usize].len() as i32;
            (0..width)
                .map(|j| {
                    if minefield[i as usize].as_bytes()[j as usize] == b'*' {
                        return '*';
                    }
                    match (0..angle.len() - 1)
                        .map(|idx| (i + angle[idx], j + angle[idx + 1]))
                        .filter(|&(x, y)| x >= 0 && x < height && y >= 0 && y < width)
                        .filter(|&(x, y)| minefield[x as usize].as_bytes()[y as usize] == b'*')
                        .count()
                    {
                        0 => ' ',
                        n => (n as u8 + b'0') as char,
                    }
                })
                .collect()
        })
        .collect()
}

#[test]
fn test() {
    use std::time::Instant;

    let minefield = ["   ".to_string(),
        " * ".to_string(),
        "   ".to_string()];
    let height = minefield.len() as i32;
    let width = minefield[0].len() as i32;
    let angle = [-1, 0, 1, 0, -1, -1, 1, 1, -1];
    let i = 1;
    let j = 1;

    // 测试函数式风格
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let _ = match (0..angle.len() - 1)
            .map(|idx| (i + angle[idx], j + angle[idx + 1]))
            .filter(|&(x, y)| x >= 0 && x < height && y >= 0 && y < width)
            .filter(|&(x, y)| minefield[x as usize].as_bytes()[y as usize] == b'*')
            .count()
        {
            0 => ' ',
            n => (n as u8 + b'0') as char,
        };
    }
    println!("函数式风格: {:?}", start.elapsed());

    // 测试显式 for 循环
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let mut count = 0;
        for idx in 0..angle.len() - 1 {
            let x = i + angle[idx];
            let y = j + angle[idx + 1];
            if x >= 0 && x < height && y >= 0 && y < width && minefield[x as usize].as_bytes()[y as usize] == b'*' {
                count += 1;
            }
        }
        let _ = match count {
            0 => ' ',
            n => (n as u8 + b'0') as char,
        };
    }
    println!("显式 for 循环: {:?}", start.elapsed());
}
