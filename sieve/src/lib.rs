macro_rules! do_while {
    ($code:block, $condition:expr) => {{
        $code
        while $condition {
            $code
        }
    }};
}

macro_rules! if_true {
    ($condition:expr, $t:expr) => {
        if $condition { $t }
    };
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut res = (2..=upper_bound).collect::<Vec<u64>>();
    
    let mut index = 0;
    while index < res.len() {
        let mut j = 2;
        let mut i = index + 1;
        while i < res.len() {
            let k = j * res[index];
            if_true!(k == res[i], res[i] = 0);
            if_true!(k <= res[i], j += 1);
            i += 1;
        }
        do_while!({ index += 1; }, index < res.len() && res[index] == 0)
    }
    
    res.iter().filter(|&&x| x != 0).cloned().collect()
}
