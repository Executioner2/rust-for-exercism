/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|c| *c != '-')
        .try_fold((0, 10), |(sum, count), digit| {
            if digit == 'X' && count == 1 {
                return Ok((sum + 10 * count, count - 1));
            } else if let Some(x) = digit.to_digit(10) {
                return Ok((sum + x as i32 * count, count - 1));
            }
            Err(())
        })
        .is_ok_and(|(sum, count)| count == 0 && sum % 11 == 0)
}
