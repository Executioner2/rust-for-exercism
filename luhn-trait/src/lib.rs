pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> Luhn for T where T: ToString {
    fn valid_luhn(&self) -> bool {
        self.to_string().chars().rev().filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(acc, count), val| {
                val.to_digit(10)
                    .map(|num| if count & 1 == 1 { ((num as i32 * 2 - 1) % 9 + 1) as u32 } else { num })
                    .map(|num| (acc + num, count + 1))
            }).is_some_and(|(sum, count)| count > 1 && sum % 10 == 0)
    }
}