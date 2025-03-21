use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 { return None; }

    let mut sum = 1u64;
    let mut sqrt = (num as f64).sqrt() as u64;
    if sqrt * sqrt == num { sqrt -= 1; }

    for i in 2..=num.min(sqrt) {
        if num % i == 0 {
            sum += num / i + i;
        }
    }
    if sum == 1 { sum = 0; }

    match sum.cmp(&num) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
        Ordering::Less => Some(Classification::Deficient),
    }
}
