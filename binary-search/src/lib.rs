use std::cmp::Ordering;

/// Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index.
pub fn find<T: AsRef<[C]>, C: Ord>(array: T, key: C) -> Option<usize> {
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = left + ((right - left) >> 1);
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }

    None
}
