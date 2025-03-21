pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0.chars().rev().filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(acc, count), val| {
                val.to_digit(10)
                    .map(|num| if count & 1 == 1 { ((num as i32 * 2 - 1) % 9 + 1) as u32 } else { num })
                    .map(|num| (acc + num, count + 1))
            }).is_some_and(|(sum, count)| count > 1 && sum % 10 == 0)
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
