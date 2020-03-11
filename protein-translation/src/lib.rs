use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codon_to_protein: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_to_protein.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        (0..rna.len())
            .step_by(3)
            .map(|i| self.name_for(&rna[i..i + 3]))
            .take_while(|&x| x != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        codon_to_protein: pairs.into_iter().collect(),
    }
}
