use std::collections::HashSet;

/// For the '{word}' word find anagrams among the following words: {possible_anagrams:?}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // 计算每个词的字母频率，并将其转换为一个新的字符串
    let mut res: HashSet<&str> = HashSet::new();
    let word_upper = word.to_uppercase();
    let mut sorted_word: Vec<char> = word_upper.chars().collect();
    sorted_word.sort();

    for &s in possible_anagrams {
        let s_upper = s.to_uppercase();
        if s_upper == word_upper {
            continue;
        }
        let mut s_sorted: Vec<char> = s_upper.chars().collect();
        s_sorted.sort();
        if s_sorted == sorted_word {
            res.insert(s);
        }
    }

    res
}
