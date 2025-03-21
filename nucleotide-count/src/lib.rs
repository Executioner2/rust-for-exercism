use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = nucleotide_counts(dna)?;
    count.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]), |mut acc, c| {
        if !acc.contains_key(&c) { return Err(c) }
        *acc.get_mut(&c).unwrap() += 1;
        Ok(acc)
    })
}
