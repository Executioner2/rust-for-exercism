fn number_length(num: u32) -> u32 {
    if num == 0 {
        return 1;
    }
    ((num as f64).log10() + 1.0) as u32
}

pub fn is_armstrong_number(mut num: u32) -> bool {
    let origin = num;
    let mut res = 0;
    let n = number_length(origin);

    while num > 0 {
        let x = num % 10;
        res += x.pow(n);
        num /= 10;
    }

    res == origin
}
