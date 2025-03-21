/// Sum the multiples of all of {factors:?} which are less than {limit}
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = std::collections::HashSet::new();

    for &x in factors {
        if x == 0 {
            continue;
        }
        let mut k = 1;
        while x * k < limit {
            set.insert(x * k);
            k += 1;
        }
    }

    set.iter().sum()
}
