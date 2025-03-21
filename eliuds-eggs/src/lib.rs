/// count the eggs in {display_value}
pub fn egg_count(mut display_value: u32) -> usize {
    let mut count = 0;
    while display_value != 0 {
        if display_value & 1 == 1 {
            count += 1;
        }
        display_value >>= 1;
    }
    count
}
