extern crate num_bigint;
extern crate num_traits;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Sub};

use num_bigint::BigInt;
use num_traits::pow;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Eq, Clone)]
pub struct Decimal {
    digits: BigInt,
    decimal_index: usize,
}

impl Decimal {
    fn new(digits: BigInt, decimal_index: usize) -> Decimal {
        let mut value = Decimal {
            digits,
            decimal_index,
        };
        value.reduce();
        value
    }

    pub fn try_from(mut input: &str) -> Option<Decimal> {
        input = input.trim();
        let mut digits = String::with_capacity(input.len());
        let mut decimal_index = None;
        for ch in input.chars() {
            match ch {
                '0'..='9' | '-' | '+' => {
                    digits.push(ch);
                    if let Some(idx) = decimal_index.as_mut() {
                        *idx += 1;
                    }
                }
                '.' => {
                    if decimal_index.is_some() {
                        return None;
                    }
                    decimal_index = Some(0)
                }
                _ => return None,
            }
        }
        Some(Decimal::new(
            digits.parse::<BigInt>().ok()?,
            decimal_index.unwrap_or(0),
        ))
    }

    fn equalize_precision(one: &mut Decimal, two: &mut Decimal) {
        fn expand(lower_precision: &mut Decimal, higher_precision: &Decimal) {
            let precision_difference =
                higher_precision.decimal_index - lower_precision.decimal_index;

            lower_precision.digits =
                &lower_precision.digits * pow(BigInt::from(10_usize), precision_difference);
            lower_precision.decimal_index += precision_difference;
        }
        match one.decimal_index.cmp(&two.decimal_index) { 
            Ordering::Less => expand(one, two),
            Ordering::Greater => expand(two, one),
            Ordering::Equal => {}
        }
        assert_eq!(one.decimal_index, two.decimal_index);
    }
    
    fn reduce(&mut self) {
        let extra_zeroes = self
            .digits
            .to_string() // produce a decimal representation
            .chars()
            .rev() // trailing values
            .take(self.decimal_index) // not counting past the decimal point
            .take_while(|&c| c == '0') // counting only `0` digits
            .count();
        self.digits = &self.digits / pow(BigInt::from(10_usize), extra_zeroes);
        self.decimal_index -= extra_zeroes;
    }
}

macro_rules! auto_impl_decimal_ops {
    ($trait:ident, $func_name:ident, $digits_operation:expr, $index_operation:expr) => {
        impl $trait for Decimal {
            type Output = Self;
            fn $func_name(mut self, mut rhs: Self) -> Self {
                Decimal::equalize_precision(&mut self, &mut rhs);
                Decimal::new(
                    $digits_operation(self.digits, rhs.digits),
                    $index_operation(self.decimal_index, rhs.decimal_index),
                )
            }
        }
    };
}

auto_impl_decimal_ops!(Add, add, |s, o| s + o, |s, _| s);
auto_impl_decimal_ops!(Sub, sub, |s, o| s - o, |s, _| s);
auto_impl_decimal_ops!(Mul, mul, |s, o| s * o, |s, o| s + o);

macro_rules! auto_impl_decimal_cow {
    ($trait:ident, $func_name:ident, $digits_operation:expr, $return_type:ty) => {
        impl $trait for Decimal {
            fn $func_name(&self, other: &Self) -> $return_type {
                if self.decimal_index == other.decimal_index {
                    $digits_operation(&self.digits, &other.digits)
                } else {
                    let mut one = self.clone();
                    let mut two = other.clone();
                    Decimal::equalize_precision(&mut one, &mut two);
                    one.$func_name(&two)
                }
            }
        }
    };
}

auto_impl_decimal_cow!(PartialEq, eq, |a, b| a == b, bool);
auto_impl_decimal_cow!(Ord, cmp, |a: &BigInt, b: &BigInt| a.cmp(b), Ordering);

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let digits = format!("{:0>width$}", self.digits, width = self.decimal_index);
        if self.decimal_index == digits.len() {
            write!(f, "0.{}", digits)
        } else if self.decimal_index == 0 {
            write!(f, "{}", digits)
        } else {
            let (before_index, after_index) = digits.split_at(digits.len() - self.decimal_index);
            write!(f, "{}.{}", before_index, after_index)
        }
    }
}