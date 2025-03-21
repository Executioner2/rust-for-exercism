/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars().rev().filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(acc, count), val| {
            val.to_digit(10)
                .map(|num| if count & 1 == 1 { ((num as i32 * 2 - 1) % 9 + 1) as u32 } else { num })
                .map(|num| (acc + num, count + 1))
        }).is_some_and(|(sum, count)| count > 1 && sum % 10 == 0)
}
