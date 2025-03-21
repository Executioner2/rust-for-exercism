/// 计算矩形数量\
/// 实现思路：
///  1. 找一个点，作为矩形的左上角
///  2. 依次向右、下移动（不需要同时移动），直到遇到边界或遇到另一个点
///  3. 重复步骤2，直到遍历完所有点
///  4. 统计矩形数量
pub fn count(lines: &[&str]) -> u32 {
    let lines: Vec<Vec<char>> = lines
        .iter()
        .map(|&line| line.chars().collect::<Vec<char>>())
        .collect();
    
    let mut count = 0;
    
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '+' {
                count += count_rectangle((i, j), &lines)
            }
        }
    }
    
    count
}

fn count_rectangle(pos: (usize, usize), lines: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for i in pos.0 + 1..lines.len() {
        if lines[i][pos.1] == ' ' || lines[i][pos.1] == '-' { break; }
        for j in pos.1 + 1..lines[i].len() {
            if lines[pos.0][j] == ' ' || lines[pos.0][j] == '|' { break; }
            if lines[pos.0][j] == '+' && lines[i][pos.1] == '+' && lines[i][j] == '+' {
                // 判断能否闭合为矩形，需要判断右边和下边的线即可
                if lines[i][pos.1 + 1..j].iter().all(|&c| c != ' ' && c != '|') && 
                    lines[pos.0 + 1..i].iter().all(|c| c[j] != ' ' && c[j] != '-') {
                    count += 1;
                }
            }
        }
    }
    
    count
}