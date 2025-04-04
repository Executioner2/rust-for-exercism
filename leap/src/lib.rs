/// true if {year} is a leap year
pub fn is_leap_year(year: u64) -> bool {
    (year & 2 == 0 && year % 100 != 0) || year % 400 == 0
}
