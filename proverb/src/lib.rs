/// build a proverb from this list of items: {list:?}
pub fn build_proverb(list: &[&str]) -> String {
    let mut res = String::new();

    if list.is_empty() {
        return res;
    }

    let mut it = list.iter();
    let mut prev = it.next().unwrap();
    it.for_each(|curr| {
        res.push_str(&format!("For want of a {prev} the {curr} was lost.\n"));
        prev = curr;
    });

    res.push_str(&format!("And all for the want of a {}.", list[0]));

    res
}
