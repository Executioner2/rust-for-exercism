use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    let lower = words.to_lowercase();
    let slice: &str = lower.as_ref();
    for word in slice
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|s| !s.is_empty())
    {
        let x= word.trim_matches('\'').to_lowercase();
        if !x.is_empty() {
            *map.entry(x).or_insert(0) += 1;
        }
    }
    map
}
