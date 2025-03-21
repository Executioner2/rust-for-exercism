use std::cmp::Ordering;
use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut map = HashMap::new();
    let mut col_min: Vec<Vec<usize>> = vec![vec![]; input[0].len()]; // 列最小值

    // 寻找行最大和列最小
    for i in 0..input.len() {
        let mut ars = vec![0];
        for j in 0..input[i].len() {
            
            // 这一段换成 if else 更简洁，但 Clippy 会给出警告
            match input[i][j].cmp(&input[i][ars[0]]) { 
                Ordering::Greater => {
                    ars.clear();
                    ars.push(j);    
                } Ordering::Equal => ars.push(j),
                _ => {}
            }
            
            if col_min[j].is_empty() || input[i][j] < input[col_min[j][0]][j] {
                col_min[j].clear();
                col_min[j].push(i);
            } else if input[i][j] == input[col_min[j][0]][j] {
                col_min[j].push(i);
            }
        }
        for j in ars {
            map.insert((i, j), 1);
        }
    }

    // 遍历最小
    for (j, col) in col_min.iter().enumerate() {
        for &i in col {
            *map.entry((i, j)).or_insert(0) += 1;
        }
    }

    map.iter().filter(|&(_, &val)| val > 1).map(|(key, _)| *key).collect()
}
