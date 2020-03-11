#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

fn check_seq(dna: &str, nucleotides: &'static str) -> Result<String, usize> {
    dna.chars()
        .enumerate()
        .map(|(i, c)| {
            if nucleotides.contains(c) {
                Ok(c)
            } else {
                Err(i)
            }
        })
        .collect()
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        check_seq(dna, "GCTA").map(DNA)
    }

    pub fn into_rna(self) -> RNA {
        RNA(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        check_seq(rna, "CGAU").map(RNA)
    }
}
