use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    val: u64,
    ranges: HashSet<(u64, u64)>
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.val
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.ranges.clone()
    }
}

/// returns the minimum palindrome and maximum palindrome of the products of two factors in the range {min} to {max}
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min >= max {
        return None;
    }

    let left = min * min;
    let right = max * max;

    let min_palindrome = find_palindrome((min, max), left..=right)?;
    let max_palindrome = find_palindrome((min, max), (left..=right).rev())?;

    Some((min_palindrome, max_palindrome))
}

fn find_palindrome(range: (u64, u64), it: impl Iterator<Item=u64>) -> Option<Palindrome> {
    let is_palindrome = |n: u64| -> bool {
        let mut k = n;
        let mut new = 0;
        while k > 0 {
            new = new * 10 + k % 10;
            k /= 10;
        };
        new == n
    };

    for i in it {
        if is_palindrome(i) {
            if let Some(range) = find_range(i, range) {
                return Some(Palindrome {
                    val: i,
                    ranges: range
                });
            }
        }
    }

    None
}

fn find_range(n: u64, (min,  max): (u64, u64)) -> Option<HashSet<(u64, u64)>> {
    let mut factors = HashSet::new();
    let limit = (n as f64).sqrt() as u64;

    for a in min..=max.min(limit) {
        if n % a == 0 {
            let b = n / a;
            if b >= min && b <= max {
                factors.insert((a, b));
            }
        }
    }

    if factors.is_empty() {
        None
    } else {
        Some(factors)
    }
}
