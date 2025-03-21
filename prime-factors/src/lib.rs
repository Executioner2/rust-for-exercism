/// This should calculate the prime factors of {n}
pub fn factors(mut n: u64) -> Vec<u64> {
    let mut ans = Vec::new();

    let mut f = 2;
    while f <= n {
        while n % f == 0 {
            ans.push(f);
            n /= f;
        }
        f += 1;
    }

    ans
}
