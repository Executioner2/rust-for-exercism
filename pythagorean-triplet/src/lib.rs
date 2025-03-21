use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();

    for a in 1..=sum / 3 {
        for b in a + 1..=sum / 2 {
            let c = sum - a - b;
            if c <= b {
                break;
            }
            if a * a + b * b == c * c {
                result.insert([a, b, c]);
            }
        }
    }

    result
}
