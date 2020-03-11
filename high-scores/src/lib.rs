#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.into(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = Vec::with_capacity(3);
        self.scores.iter().cloned().for_each(|x| {
            let idx = v.iter().position(|&y| x <= y).unwrap_or_else(|| v.len());
            if v.len() < 3 {
                v.insert(idx, x)
            } else if idx != 0 {
                v.insert(idx, x);
                v.remove(0);
            }
        });
        v.reverse();
        v
    }
}
