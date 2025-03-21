use std::collections::HashMap;

thread_local! {static TRANSLATION_TABLE: HashMap<&'static str, &'static str> = HashMap::from([
    ("AUG", "Methionine"),
    ("UUU", "Phenylalanine"),
    ("UUC", "Phenylalanine"),
    ("UUA", "Leucine"),
    ("UUG", "Leucine"),
    ("UCU", "Serine"),
    ("UCC", "Serine"),
    ("UCA", "Serine"),
    ("UCG", "Serine"),
    ("UAU", "Tyrosine"),
    ("UAC", "Tyrosine"),
    ("UGU", "Cysteine"),
    ("UGC", "Cysteine"),
    ("UGG", "Tryptophan"),
    ("UAA", "STOP"),
    ("UAG", "STOP"),
    ("UGA", "STOP"),
]);}

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut result = Vec::new();
    let mut chars = rna.chars();
    while let (Some(c1), c2, c3) = (chars.next(), chars.next(), chars.next()) {
        if c2.is_none() || c3.is_none() {
            return None;
        }
        let (c2, c3) = (c2.unwrap(), c3.unwrap());
        if [c1, c2, c3].iter().any(|&c| c != 'A' && c != 'U' && c != 'C' && c != 'G') {
            return None;
        }
        let codon = [c1, c2, c3].iter().collect::<String>();
        match TRANSLATION_TABLE.with(|t| t.get(codon.as_str()).copied()) {
            Some("STOP") => return Some(result),
            Some(amino_acid) => result.push(amino_acid),
            _ => {},
        }
    }
    Some(result)
}
