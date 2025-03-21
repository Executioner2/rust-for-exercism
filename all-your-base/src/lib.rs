use crate::Error::InvalidDigit;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    // 先转十进制
    let decimal = number
        .iter()
        .rev()
        .enumerate()
        .try_fold(0, |acc, (i, &digit)| {
            if digit >= from_base {
                return Err(InvalidDigit(digit));
            }
            Ok(acc + digit * from_base.pow(i as u32))
        })?;

    // 再转目标进制
    let mut result = vec![];
    let mut remainder = decimal;
    while remainder > 0 {
        result.push(remainder % to_base);
        remainder /= to_base;
    }
    result.reverse();

    if result.is_empty() {
        result.push(0);
    }

    Ok(result)
}
