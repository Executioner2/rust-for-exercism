#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

const RNA: [char; 4] = ['C', 'G', 'A', 'U'];
const DNA: [char; 4] = ['G', 'C', 'T', 'A'];

fn validate(s: &str, chars: &[char; 4]) -> Result<String, usize> {
    match s.chars().position(|c| !chars.contains(&c)) {
        Some(i) => Err(i),
        None => Ok(s.to_string()),
    }
}

impl Dna {
    /// Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate(dna, &DNA).map(Self)
    }

    /// Transform Dna {self:?} into corresponding Rna
    pub fn into_rna(self) -> Rna {
        Rna::from(self)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate(rna, &RNA).map(Self)
    }
}

impl From<Dna> for Rna {
    fn from(dna: Dna) -> Self {
        let transform = dna.0.chars().map(|c| {
            RNA[DNA.iter().position(|&dc| dc == c).unwrap()]
        }).collect::<String>();
        Self(transform)
    }
}
