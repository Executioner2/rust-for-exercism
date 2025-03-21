const UNITS: [&str; 7] = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];
const BELOW_20: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub fn encode(num: u64) -> String {
    if num == 0 {
        return String::from("zero");
    }
    let mut words = String::new();
    let mut chunks = Vec::new();
    let mut n = num;

    while n > 0 {
        chunks.push(n % 1000);
        n /= 1000;
    }

    for (i, &chunk) in chunks.iter().enumerate().rev() {
        if chunk != 0 {
            words.push_str(&convert_chunk(chunk));
            words.push(' ');
            words.push_str(UNITS[i]);
            words.push(' ');
        }
    }

    words.trim().to_string()
}

fn convert_chunk(num: u64) -> String {
    let mut words = String::new();
    if num >= 100 {
        words.push_str(BELOW_20[(num / 100) as usize]);
        words.push_str(" hundred ");
        words.push_str(&convert_chunk(num % 100));
    } else if num >= 20 {
        words.push_str(TENS[(num / 10) as usize]);
        if num % 10 != 0 { words.push('-'); }
        words.push_str(&convert_chunk(num % 10));
    } else if num > 0 {
        words.push_str(BELOW_20[num as usize]);
    }
    words.trim().to_string()
}
