use std::collections::HashMap;

fn check(nucleotide: char) -> Result<char, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => Ok(nucleotide),
        _ => Err(nucleotide),
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let nucleotide = check(nucleotide)?;
    dna.chars()
        .map(|c| check(c).map(|c| (c == nucleotide) as usize))
        .sum()
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut ans: HashMap<_, _> = "ACGT".chars().map(|c| (c, 0)).collect();
    for c in dna.chars().map(check) {
        *ans.entry(c?).or_default() += 1;
    }
    Ok(ans)
}
