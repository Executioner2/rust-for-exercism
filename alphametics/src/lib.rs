use std::collections::{HashMap, HashSet};

/// 用回溯解决就行
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = parse(input);
    let mut ans = vec![0; input.len()];
    let nums: Vec<u8> = (0..10).collect();
    backtracking(0, &mut ans, &nums, &input)
}

/// 解析字符串
fn parse(input: &str) -> Vec<(char, i64, bool)> {
    let mut map = HashMap::new();
    let mut leading = HashSet::new(); // 用于记录这个字符是否出现过在首位
    let mut rate: i64 = -1; // 从后往前解析，res部分为负数，方便快速计算相等式是否成立
    let mut prev = ' ';

    input.chars().rev().for_each(|c| {
        match c {
            'A'..='Z' => {
                // 表示当前字符，在第 rate 位上出现过。如果重复出现，则会加上原来的
                *map.entry(c).or_insert(0) += rate;
                rate *= 10;
                prev = c;
            }
            _ => {
                if rate.abs() > 1 {
                    leading.insert(prev); // 忽略单个 0 的情况
                }
                rate = 1;
            }
        }
    });

    if rate.abs() > 1 {
        leading.insert(prev);
    }
    map.iter()
        .map(|(&k, &v)| (k, v, leading.contains(&k)))
        .collect()
}

/// 回溯查询
fn backtracking(
    count: usize,
    ans: &mut [u8],
    nums: &[u8],
    input: &[(char, i64, bool)],
) -> Option<HashMap<char, u8>> {
    if count == ans.len() {
        return if quick_equals(ans, input) {
            Some(
                ans.iter()
                    .zip(input.iter())
                    .map(|(&num, &(ch, _, _))| (ch, num))
                    .collect(),
            )
        } else {
            None
        };
    }

    for (i, &num) in nums.iter().enumerate() {
        if num == 0 && input[count].2 {
            // 存在长度不为1的首位0
            continue;
        }
        // 这里一定不要用标记法。标记法实际上还是会有遍历次数
        let mut new_nums = nums.to_vec();
        new_nums.remove(i);
        ans[count] = num;
        if let Some(res) = backtracking(count + 1, ans, &new_nums, input) {
            return Some(res);
        }
    }

    None
}

/// 快速判断是否等于。通过预处理，可以直接将所有相加，最终为0，则表示相等式成立。
fn quick_equals(ans: &mut [u8], input: &[(char, i64, bool)]) -> bool {
    let mut result = 0;
    for (i, &x) in ans.iter().enumerate() {
        result += x as i64 * input[i].1;
    }
    result == 0
}
