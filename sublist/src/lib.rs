#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Determine if the {first_list:?} is equal to, sublist of, superlist of or unequal to {second_list:?}.
pub fn sublist(a: &[i32], b: &[i32]) -> Comparison {
    use Comparison::*;
    let f = |a: &[i32], b: &[i32]| a.is_empty() || b.windows(a.len()).any(|w| w == a);
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (n, m) if n < m => if f(a, b) { Sublist } else { Unequal },
        (n, m) if n > m => if f(b, a) { Superlist } else { Unequal },
        (_, _) => if a == b { Equal } else { Unequal }
    }
}
