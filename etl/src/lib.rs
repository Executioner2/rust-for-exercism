use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res = BTreeMap::new();

    for (&num, list) in h.iter() {
        list.iter().for_each(|c| {
            res.insert(c.to_ascii_lowercase(), num);
        })
    }

    res
}
