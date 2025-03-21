/// What is the 0-indexed {n}th prime number?
pub fn nth(n: u32) -> u32 {
    let is_prime = |x, ars: &[u32]| -> bool {
        let sqrt_n = (x as f64).sqrt() as u32;
        for &a in ars {
            if a > sqrt_n {
                break;
            }
            if x % a == 0 {
                return false;
            }
        }
        true
    };

    let mut i: u32 = 3;
    let mut ars = vec![2];
    while ars.len() <= n as usize {
        if is_prime(i, &ars) {
            ars.push(i);
        }
        i += 2; // 只需要检查奇数
    }

    ars[n as usize]
}
