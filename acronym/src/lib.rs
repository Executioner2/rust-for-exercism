/// Given the phrase '{phrase}', return its acronym
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split([' ', '-'])
        .filter(|&x| !x.trim().is_empty())
        .flat_map(|s| {
            let mut prev = ' ';
            s.chars()
                .filter(|c| c.is_alphabetic())
                .filter_map(move |c| {
                    let res =
                        if prev == ' ' || (c.is_ascii_uppercase() && prev.is_ascii_lowercase()) {
                            Some(c.to_ascii_uppercase())
                        } else {
                            None
                        };
                    prev = c;
                    res
                })
        })
        .collect()
}